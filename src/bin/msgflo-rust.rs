
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
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
struct ParticipantPort {
    id: String, // port name
    queue: String, // the associated message queue
    // FIXME: support. Is a keyword so needs some special handling   type: String, // datatype, ex: "boolean"
    // options: queue options as specified by the message queue implementation   
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

fn send_discovery(channel: &mut Channel, info: &ParticipantInfo) {
    let queue_name = "fbp"; // TODO: use an exchange istead, requires protocol change in msgflo

    let queue_declare = channel.queue_declare(queue_name, false, true, false, false, false, Table::new());
    let content_type = Some("application/json".to_string());
    let props = protocol::basic::BasicProperties { content_type: content_type, ..Default::default() };

    let payload = json::encode(&info).unwrap();
    println!("sending: {}", payload);
    let res = channel.basic_publish("", queue_name, true, false, props, payload.into_bytes());
}

// for debugging
fn selftest() {
    let mut session = Session::new(Options{vhost: "/", .. Default::default()}).ok().expect("Can't create session");
    let mut channel = session.open_channel(1).expect("channel");

    let info = ParticipantInfo {
        id: "part11".to_string(),
        role: "myrole".to_string(),
        component: "rust/First".to_string(),
        inports: Vec::<ParticipantPort>::new(),
        outports: Vec::<ParticipantPort>::new(),
    };

    send_discovery(&mut channel, &info);
    listen_discovery(&mut channel);

    channel.start_consuming();

    let closed = channel.close(200, "Bye".to_string());
    session.close(200, "Good Bye".to_string());
}

fn main() {
    selftest();
}
