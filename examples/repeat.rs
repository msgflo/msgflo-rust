
extern crate msgflo;

use msgflo::{ParticipantInfo, Participant, ParticipantPort};

struct Repeater {
    data: str
}

impl msgflo::Participant for Repeater {

    fn info() {
        // MAYBE Builder could be passed in??
        ParticipantInfoBuilder::new("rust/Repeat")
            .label("Repeats input as-is")
            //.icon("fa-car")
            .inport("in")
                .datatype("object")
                .description("Input")
                //.queue("some queue")
                .up()
            .outport("out").up()
            .info()
    }

    // TODO: how to access sending function, for sending at arbitrary times?
    fn process(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        println!("Repeater({}) repeating {}", self.data);
        return Ok(input);
    }
}


fn main() {
    let p = Repeater { data: "party" };
    msgflo::participant_main(p);
}
