
extern crate amqp;

use amqp::{ConsumerCallBackFn, Session, Table, Basic, Channel, Options};
use amqp::protocol;
use std::default::Default;

// for debugging
fn listen_discovery(channel: &mut Channel) {
    fn participant_discovered(channel: &mut Channel, deliver: protocol::basic::Deliver,
                            headers: protocol::basic::BasicProperties, body: Vec<u8>){
        println!("Discovered MsgFlo participant !!:");
        //println!("Deliver info: {:?}", deliver);
        //println!("Content headers: {:?}", headers);
        //println!("Content body: {:?}", body);
        channel.basic_ack(deliver.delivery_tag, false);
    }

    let queue_name = "fbp";
    let consumer_name = channel.basic_consume(participant_discovered as ConsumerCallBackFn, queue_name,
                                                "", false, false, false, false, Table::new());
}

fn send_discovery(channel: &mut Channel) {
    let queue_name = "fbp"; // TODO: use an exchange istead, requires protocol change in msgflo

    let queue_declare = channel.queue_declare(queue_name, false, true, false, false, false, Table::new());
    let content_type = Some("application/json".to_string());
    let props = protocol::basic::BasicProperties { content_type: content_type, ..Default::default() };

    let payload = b"Hello from rust!";
    let res = channel.basic_publish("", queue_name, true, false, props, payload.to_vec());
}

// for debugging
fn selftest() {
    let mut session = Session::new(Options{vhost: "/", .. Default::default()}).ok().expect("Can't create session");
    let mut channel = session.open_channel(1).expect("channel");

    send_discovery(&mut channel);
    listen_discovery(&mut channel);

    channel.start_consuming();

    let closed = channel.close(200, "Bye".to_string());
    session.close(200, "Good Bye".to_string());
}

fn main() {
    selftest();
}
