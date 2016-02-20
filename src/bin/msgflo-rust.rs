
extern crate rustc_serialize;
extern crate amqp;

use amqp::{ConsumerCallBackFn, Session, Table, Basic, Channel, Options};
use amqp::protocol;
use std::default::Default;
use rustc_serialize::json;
use std::slice;

// for debugging
fn listen_discovery(channel: &mut Channel) {
    fn participant_discovered(channel: &mut Channel, deliver: protocol::basic::Deliver,
                            headers: protocol::basic::BasicProperties, body: Vec<u8>){
        let s = std::str::from_utf8(&body).unwrap();
        let info: ParticipantInfo = json::decode(s).unwrap();
        println!("MsgFlo participant discovered: {:?}", info);
        //println!("Deliver info: {:?}", deliver);
        //println!("Content headers: {:?}", headers);
        //println!("Content body: ", );
        channel.basic_ack(deliver.delivery_tag, false);
    }

    let queue_name = "fbp";
    let consumer_name = channel.basic_consume(participant_discovered as ConsumerCallBackFn, queue_name,
                                                "", false, false, false, false, Table::new());
    println!("listening for discovery messages");
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
struct ParticipantPort {
    id: String, // port name
    queue: String, // the associated message queue
    // FIXME: support. Is a keyword so needs some special handling   type: String, // datatype, ex: "boolean"
    // options: queue options as specified by the message queue implementation   
    // description?? TODO: standardize
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
struct ParticipantInfo {
    id: String, // unique name
    role: String ,// role participant has
    component: String, // component the participant is instance of
//   label: Option<String>, // (optional) short human-readable description
//    icon: Option<String>, // (optional)
    inports: Vec<ParticipantPort>,
    outports: Vec<ParticipantPort>,
}

type ProcessFunction = fn(i32) -> i32;
struct Participant {
    info: ParticipantInfo,
    process: ProcessFunction,
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
    println!("sending on {}: {}", queue_name, payload);
    let res = channel.basic_publish("", queue_name, true, false, props, payload.into_bytes());
    res.expect("send on new queue failed");
}

fn send_discovery(channel: &mut Channel, info: &ParticipantInfo) {
    let queue_name = "fbp"; // TODO: use an exchange istead, requires protocol change in msgflo

    let payload = json::encode(&info).unwrap();
    create_queue_and_send(channel, queue_name, payload);
}

// FIXME: actually call ProcessFunction
fn setup_inport(port: &ParticipantPort, connection: &mut Connection) {
    println!("setup inport: {}", port.queue.to_string());

    fn on_message(channel: &mut Channel, deliver: protocol::basic::Deliver,
                            headers: protocol::basic::BasicProperties, body: Vec<u8>) {

        println!("!! got msg on inport");
        let s = std::str::from_utf8(&body).unwrap();
        let info: ParticipantInfo = json::decode(s).unwrap();
        println!("got message on: {:?}", info);
        channel.basic_ack(deliver.delivery_tag, false);
    }

    // create
    let declare = connection.channel.queue_declare(port.queue.to_string(), false, true, false, false, false, Table::new());
    declare.expect("inport queue creation failed");

    // subscribe
    let q = port.queue.to_string();
    let cons = connection.channel.basic_consume(on_message as ConsumerCallBackFn, q,
                                                "".to_string(), false, false, false, false, Table::new());

    cons.expect("iport setup failed");

    println!("setup inport done: {:?}", port.id.to_string());
}

fn setup_outport(port: &ParticipantPort, connection: &mut Connection) {

    let queue_declare = connection.channel.queue_declare(port.queue.to_string(), false, true, false, false, false, Table::new());
    let content_type = Some("application/json".to_string());
}

// FIXME: actually send
fn send_output(participant: &Participant) {

    let queue_name = participant.info.outports[0].queue.to_string();
//    let payload = json::encode(&info).unwrap();
//    println!("sending: {}", payload);
//    let props = protocol::basic::BasicProperties { content_type: content_type, ..Default::default() };
//    let res = channel.basic_publish("", port.queue.to_string(), true, false, props, payload.into_bytes());
}


fn start_participant(participant: &Participant) -> Connection {
    let mut session = Session::new(Options{vhost: "/", .. Default::default()}).ok().expect("Can't create session");
    let mut channel = session.open_channel(1).expect("channel");

    send_discovery(&mut channel, &participant.info);

    let mut conn = Connection { session: session, channel: channel };

    // XXX: seems like can only connect to one queue??
    listen_discovery(&mut conn.channel); // TESTING

    setup_inport(&participant.info.inports[0], &mut conn);


    return conn;
}

// for debugging
fn stop_participant(participant: &Participant, connection: &mut Connection) {

    let closed = connection.channel.close(200, "Bye".to_string());
    connection.session.close(200, "Good Bye".to_string());
}

// TODO: wire up input/output to the process() function
// TODO: respect MSGFLO_BROKER envvar
// TODO: setup the msgflo hetro automated test
// TODO: nicer way to declare ports
fn main() {
    let info = ParticipantInfo {
        id: "part11".to_string(),
        role: "myrole".to_string(),
        component: "rust/First".to_string(),
        inports: vec! [ ParticipantPort { id: "in".to_string(), queue: "rustparty.IN".to_string() } ],
        outports: vec! [ ParticipantPort { id: "out".to_string(), queue: "rustparty.OUT".to_string() } ],
    };
    fn process_repeat(input: i32) -> i32 {
        return input;
    }

    let p = Participant { info: info, process: process_repeat };

    let mut c = start_participant(&p);

    create_queue_and_send(&mut c.channel, "rustparty.IN", "{ \"data\": 300 }".to_string());

    c.channel.start_consuming();
    stop_participant(&p, &mut c);
}
