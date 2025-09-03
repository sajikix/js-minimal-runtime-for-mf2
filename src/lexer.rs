static RESERVED_WORDS: [&str; 4] = ["const", "function", "return", "new"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Punctuator(char),
    Number(u64),
    Identifier(String),
    Keyword(String),
    StringLiteral(String),
}

pub struct Lexer {
    pos: usize,
    input: Vec<char>,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            pos: 0,
            input: src.chars().collect(),
        }
    }

    fn consume_number(&mut self) -> u64 {
        let start = self.pos;

        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let number_str: String = self.input[start..self.pos].iter().collect();
        number_str.parse().unwrap()
    }

    fn consume_identifier(&mut self) -> String {
        let start = self.pos;

        while self.pos < self.input.len()
            && (self.input[self.pos].is_ascii_alphanumeric()
                || self.input[self.pos] == '_'
                || self.input[self.pos] == '$')
        {
            self.pos += 1;
        }

        let identifier: String = self.input[start..self.pos].iter().collect();
        // Check if it's a reserved word
        if RESERVED_WORDS.contains(&identifier.as_str()) {
            identifier // Return as is, will be treated as Keyword in Token
        } else {
            identifier // Return as is, will be treated as Identifier in Token 
        }
    }

    fn consume_string(&mut self) -> String {
        self.pos += 1; // Skip the opening quote
        let start = self.pos;

        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.pos += 1;
        }

        let string_literal: String = self.input[start..self.pos].iter().collect();
        self.pos += 1; // Skip the closing quote
        string_literal
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        while self.input[self.pos] == ' ' || self.input[self.pos] == '\n' {
            self.pos += 1;

            if self.pos >= self.input.len() {
                return None;
            }
        }

        let c = self.input[self.pos];

        let token = match c {
            '+' | '-' | ';' | '=' | '(' | ')' | '{' | '}' | ',' | '.' | ':' => {
                let t = Token::Punctuator(c);
                self.pos += 1;
                t
            }
            '0'..='9' => Token::Number(self.consume_number()),
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                let identifier = self.consume_identifier();
                if RESERVED_WORDS.contains(&identifier.as_str()) {
                    Token::Keyword(identifier)
                } else {
                    Token::Identifier(identifier)
                }
            }
            '"' => Token::StringLiteral(self.consume_string()),
            _ => unimplemented!("char {:?} is not supported yet", c),
        };

        Some(token)
    }
}
