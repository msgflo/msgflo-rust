
use argparse;
use amqp::{ConsumerCallBackFn, Session, Table, Basic, Channel, Options, Consumer};
use amqp::protocol;
use std::default::Default;
use rustc_serialize::json;
use std::slice;


#[derive(Debug, Default, RustcDecodable, RustcEncodable, Clone)]
pub struct ParticipantPort {
    pub id: String, // port name
    pub queue: String, // the associated message queue
    // FIXME: support. Is a keyword so needs some special handling   type: String, // datatype, ex: "boolean"
    // options: queue options as specified by the message queue implementation   
    // description?? TODO: standardize
}


#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ParticipantInfo {
    pub id: String, // unique name
    pub role: String ,// role participant has
    pub component: String, // component the participant is instance of
    pub label: Option<String>, // (optional) short human-readable description
    pub icon: Option<String>, // (optional)
    pub inports: Vec<ParticipantPort>,
    pub outports: Vec<ParticipantPort>,
}

pub struct Component {
    component: String,
    label: Option<String>,
    icon: Option<String>,
    inports: Vec<ParticipantPort>,
    outports: Vec<ParticipantPort>,
}

impl Component {
    pub fn new(component: &str) -> Component {
        Component {
            component: component.to_string(),
            label: None,
            icon: None,
            inports: vec! [],
            outports: vec! [],
        }
    }

    pub fn label(&mut self, label: &str) -> &mut Component {
        self.label = Some(label.to_string());
        self
    }

    pub fn inport(&mut self, id: &str) -> &mut Component {
        let port = ParticipantPort { id: id.to_string(), queue: "".to_string() };
        self.inports.push(port);
        self
    }
    pub fn outport(&mut self, id: &str) -> &mut Component {
        let port = ParticipantPort { id: id.to_string(), queue: "".to_string() };
        self.outports.push(port);
        self
    }
    pub fn info(&self) -> ParticipantInfo {
        ParticipantInfo {
            id: "".to_string(), // FIXME: use Option
            role: "".to_string(), // FIXME: use Option
            icon: None,
            label: None,
            component: self.component.to_string(),
            inports: self.inports.clone(),
            outports: self.outports.clone(),
        }
    }
}


type SendFunction = fn(String, Vec<u8>);
pub type ProcessFunction = fn(Vec<u8>) -> Result<Vec<u8>, Vec<u8>>;
pub struct Participant {
    pub info: ParticipantInfo,
    pub process: ProcessFunction,
}

struct Connection {
    session: Session,
    channel: Channel,
}

// FIXME: wrap in protocol/command/payload envelope
fn create_queue_and_send(channel: &mut Channel, queue_name: &str, payload: String) {

    let queue_declare = channel.queue_declare(queue_name, false, true, false, false, false, Table::new());
    queue_declare.expect("queue creation failed");
    let content_type = Some("application/json".to_string());
    let props = protocol::basic::BasicProperties { content_type: content_type, ..Default::default() };
    debug!("sending on {}: {}", queue_name, payload);
    let res = channel.basic_publish("", queue_name, true, false, props, payload.into_bytes());
    res.expect("send on new queue failed");
}

fn send_discovery(channel: &mut Channel, info: &ParticipantInfo) {
    let queue_name = "fbp"; // TODO: use an exchange istead, requires protocol change in msgflo

    let payload = json::encode(&info).unwrap();
    create_queue_and_send(channel, queue_name, payload);
    info!("sent participant discovery: {}  {}({})", info.id, info.role, info.component);
}

struct PortConsumer {
    process: ProcessFunction,
    portname: String,
    outqueue: String, // FIXME: allow sending on any port, also multiple times
}

fn send_out(channel: &mut Channel, exchange: String, data: Vec<u8>) {

    let routing_key = "".to_string();
    let content_type = Some("application/json".to_string()); // TODO: should be parameter
    let props = protocol::basic::BasicProperties { content_type: content_type, ..Default::default() };
    let s = channel.basic_publish(exchange, routing_key, true, false, props, data);
    s.expect("failed to send");
    debug!("sent output");
}

impl Consumer for PortConsumer {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       headers: protocol::basic::BasicProperties,
                       body: Vec<u8>) {

        debug!("calling process()");
        let f = self.process;
        let res = f(body);
        debug!("process() returned");

        if res.is_ok() {
            debug!("ACKing and sending");
            send_out(channel, self.outqueue.to_string(), res.unwrap());
            let r = channel.basic_ack(deliver.delivery_tag, false);
        } else {
            error!("process() errored");
            let r = channel.basic_nack(deliver.delivery_tag, true, true);
        }
        // FIXME: send error data
    }
}

// FIXME: actually call ProcessFunction
fn setup_inport(participant: &Participant, port: &ParticipantPort, connection: &mut Connection) {
    debug!("setup inport: {}", port.queue.to_string());

    let consumer = PortConsumer {
        process: participant.process,
        portname: port.id.to_string(),
        outqueue: participant.info.outports[0].queue.to_string(),
    };

    // create
    let declare = connection.channel.queue_declare(port.queue.to_string(), false, true, false, false, false, Table::new());
    declare.expect("inport queue creation failed");

    // subscribe
    let q = port.queue.to_string();
    let cons = connection.channel.basic_consume(consumer, q,
                                                "".to_string(), false, false, false, false, Table::new());

    cons.expect("iport setup failed");

    debug!("inport setup done: {:?}, {:?}", port.id.to_string(), port.queue.to_string());
}

fn setup_outport(participant: &Participant, port: &ParticipantPort, connection: &mut Connection) {

    let exchange_type = "fanout".to_string();
    let declare = connection.channel.exchange_declare(port.queue.to_string(), exchange_type,
                                                    false, true, false, false, false, Table::new());
    declare.expect("outport setup failed");

    debug!("setup outport done: {:?}, {:?}", port.id.to_string(), port.queue.to_string());
}


fn start_participant(participant: &Participant, options: &ParticipantOptions) -> Connection {

    let mut session = Session::open_url(&options.broker).expect("Can't create AMQP session");
    let mut channel = session.open_channel(1).expect("could not open AMQP channel");

    let mut conn = Connection { session: session, channel: channel };

    // setup ports
    setup_inport(&participant, &participant.info.inports[0], &mut conn);
    setup_outport(&participant, &participant.info.outports[0], &mut conn);

    // send MsgFlo participant discovery message
    send_discovery(&mut conn.channel, &participant.info);

    return conn;
}

fn stop_participant(participant: &Participant, connection: &mut Connection) {

    let closed = connection.channel.close(200, "Bye".to_string());
    connection.session.close(200, "Good Bye".to_string());
}

#[derive(Debug)]
struct ParticipantOptions {
    role: String,
    broker: String,
}

impl Default for ParticipantOptions {
    fn default() -> ParticipantOptions { 
        use rand::{thread_rng, Rng};

        let id: String = thread_rng().gen_ascii_chars().take(5).collect();
        ParticipantOptions {
            broker: "amqp://localhost//".to_string(),
            role: "".to_string(), // TODO: allow anonymous, format!("msgflo-rust-{}", id),
        }
    }
}

fn parse(options: &mut ParticipantOptions) {

    use argparse::{StoreTrue, Store};
    let mut parser = argparse::ArgumentParser::new();

    parser.refer(&mut options.role)
        .add_option(&["--role"], Store, "Participant role name");
    parser.refer(&mut options.broker)
        .add_option(&["--broker"], Store, "Address of messaging broker")
        .envvar("MSGFLO_BROKER");

    parser.parse_args_or_exit(); // XXX: should return out
} 

// XXX: seems rust-amqp makes program hangs forever if error occurs / channel is borked?
// TODO: pass port info in/out of process()
// TODO: nicer way to declare ports? ideally they are enums not stringly typed?
pub fn main(p: Participant) {

    let mut options = ParticipantOptions { .. Default::default() };
    parse(&mut options);

    if options.role == "" {
        options.role = p.info.role.to_string();
    }

    let mut c = start_participant(&p, &options);
    println!("{}({}) started", &options.role, &p.info.component);
    c.channel.start_consuming();

    stop_participant(&p, &mut c);
}
    
