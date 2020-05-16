

use crate::tokens::Token;

struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source: source, tokens: Vec::<Token>::new() }
    }
}