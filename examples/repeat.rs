extern crate msgflo;

use msgflo::participant::{Participant, InfoBuilder, Info};

struct Repeat {
    state: Option<String>, // we don't really have any state
}

impl Participant for Repeat {
    fn info(&self) -> Info {
        InfoBuilder::new("rust/Repeat")
            .label("Repeats input as-is")
            .inport("in")
            .outport("out")
            .build()
    }

    fn process(&self, input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        println!("repeat process():");
        return Ok(input);
    }

}

fn main() {
    static r: Repeat = Repeat { state: None };
    msgflo::participant::main(&r);
}
