
extern crate msgflo;

use msgflo::participant::{Participant, InfoBuilder};

fn main() {
    let info =  InfoBuilder::new("rust/Repeat")
        .label("Repeats input as-is")
        .inport("in")
        .outport("out")
        .build();

    fn process_repeat(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        println!("process_repeat:");
        return Ok(input);
    }

    let p = Participant { info: info, process: process_repeat };

    msgflo::participant::main(p);
}
