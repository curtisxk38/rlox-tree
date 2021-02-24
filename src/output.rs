use crate::tree_walker::Value;


#[derive(Debug)]
pub(crate) struct Printer {

}

impl Printer {
    pub fn new() -> Printer {
        Printer{}
    }

    pub fn output_value(&mut self, value: Value) {
        println!("{}", value);
    }
}


#[derive(Debug)]
pub(crate) struct Recorder {
    pub outputted: Vec<String>
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder {outputted: Vec::new()}
    }
    pub fn output_value(&mut self, value: Value) {
        self.outputted.push(format!("{}", value));
    }
}