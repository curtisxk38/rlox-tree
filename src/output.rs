use crate::tree_walker::Value;


#[derive(Debug)]
pub(crate) struct Printer {

}

// We're allowing dead code, so that warnings aren't generated
// however the code isn't actually dead. I guess rustc/rust-analyzer can't tell that
// because we're using conditional compilation. (see tree_walker.rs)
#[allow(dead_code)]
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

#[allow(dead_code)]
impl Recorder {
    pub fn new() -> Recorder {
        Recorder {outputted: Vec::new()}
    }
    pub fn output_value(&mut self, value: Value) {
        self.outputted.push(format!("{}", value));
    }
}