use rphonetic::DaitchMokotoffSoundex;
use tantivy::tokenizer::{BoxTokenStream, Token, TokenStream};

pub struct DaitchMokotoffTokenStream<'a> {
    pub tail: BoxTokenStream<'a>,
    pub encoder: DaitchMokotoffSoundex,
    pub branching: bool,
    pub codes: Vec<String>,
    pub inject: bool,
}

impl<'a> TokenStream for DaitchMokotoffTokenStream<'a> {
    fn advance(&mut self) -> bool {
        while self.codes.is_empty() {
            let result = self.tail.advance();
            if !result {
                return false;
            }
            if self.tail.token().text.is_empty() {
                return true;
            }

            self.codes = self
                .encoder
                .inner_soundex(&self.tail.token().text, self.branching)
                .iter()
                .filter(|v| !v.is_empty())
                .cloned()
                .collect();

            if self.inject {
                return true;
            }
        }

        let code = self.codes.pop();
        match code {
            Some(code) => {
                self.tail.token_mut().text = code;
                true
            }
            None => false,
        }
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}
