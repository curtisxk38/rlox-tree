use crate::tree_walker::Value;

pub(crate) trait Outputter {
    fn output_value(&mut self, value: Value);
}

#[derive(Debug)]
pub(crate) struct Printer {

}

impl Outputter for Printer {
    fn output_value(&mut self, value: Value) {
        println!("{}", value);
    }
}


#[derive(Debug)]
pub(crate) struct Recorder {
    pub outputted: Vec<String>
}

impl Outputter for Recorder {
    fn output_value(&mut self, value: Value) {
        self.outputted.push(format!("{}", value));
    }
}