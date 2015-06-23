
extern crate amqp;

use amqp::session::Session;
use amqp::protocol;
use amqp::table;
use amqp::basic::Basic;
use amqp::channel::Channel;
use std::default::Default;


fn consumer_function(channel: &mut Channel, deliver: protocol::basic::Deliver, headers: protocol::basic::BasicProperties, body: Vec<u8>){
    println!("Got a delivery:");
    println!("Deliver info: {:?}", deliver);
    println!("Content headers: {:?}", headers);
    println!("Content body: {:?}", body);
    channel.basic_ack(deliver.delivery_tag, false);
}

fn main() {
    let mut session = Session::open_url("amqp://localhost/").ok().unwrap();
   println!("Openned session:"); 
   let mut channel = session.open_channel(1).ok().unwrap();
    println!("Openned channel: {}", channel.id);

    let queue_name = "test_queue";
    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    let queue_declare = channel.queue_declare(queue_name, false, true, false, false, false, table::new());
    println!("Queue declare: {:?}", queue_declare);

    //queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    println!("Declaring consumer...");
    let consumer_name = channel.basic_consume(consumer_function, queue_name, "", false, false, false, false, table::new());
    println!("Starting consumer {:?}", consumer_name);

    let props = protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()};
    channel.basic_publish("", queue_name, true, false, props, (b"Hello from rust!").to_vec());
 
    println!("Published");

    channel.start_consuming();

    channel.close(200, "Bye".to_string());
    session.close(200, "Good Bye".to_string());
}
