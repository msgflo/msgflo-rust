
extern crate msgflo;

use msgflo::{ParticipantInfo, Participant, ParticipantPort};

fn main() {
    let info = ParticipantInfo {
        id: "part11".to_string(),
        role: "myrole".to_string(),
        component: "rust/Repeat".to_string(),
        inports: vec! [ ParticipantPort { id: "in".to_string(), queue: "rustparty.IN".to_string() } ],
        outports: vec! [ ParticipantPort { id: "out".to_string(), queue: "rustparty.OUT".to_string() } ],
    };


        //let s = std::str::from_utf8(&body).unwrap();
        //let json_obj: json::Object = json::decode(s).expect("json parse error");

    fn process_repeat(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        println!("process_repeat:");
        return Ok(input);
    }

    let p = Participant { info: info, process: process_repeat };

    msgflo::participant_main(p);
}
