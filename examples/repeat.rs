
extern crate msgflo;

use msgflo::{ParticipantInfo, Participant, ParticipantPort};

fn main() {
    let info = ParticipantInfo {
        id: "repeat113".to_string(),
        role: "repeat".to_string(),
        component: "rust/Repeat".to_string(),
        label: Some("Repeats input as-is".to_string()),
        icon: None,
        inports: vec! [ ParticipantPort { id: "in".to_string(), queue: "repeat.IN".to_string() } ],
        outports: vec! [ ParticipantPort { id: "out".to_string(), queue: "repeat.OUT".to_string() } ],
    };


        //let s = std::str::from_utf8(&body).unwrap();
        //let json_obj: json::Object = json::decode(s).expect("json parse error");

    fn process_repeat(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        println!("process_repeat:");
        return Ok(input);
    }

    let p = Participant { info: info, process: process_repeat };

    msgflo::init_logger(); // debugging
    msgflo::participant_main(p);
}
