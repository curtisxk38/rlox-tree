use std::fmt::Display;


#[derive(Debug, Clone)]
pub(crate) struct LoxClass {
    pub name: String,

}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}