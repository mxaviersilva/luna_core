#[derive(PartialEq, Debug, Copy)]
pub enum TokenType {
    Int,
    Op,
    SPC,
    EOF,
    NOP,
}

#[derive(Copy)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

/// # A parsing context for expressions
/// this context allows you to parse single expressions
pub struct Context {
    text: Vec<char>,
    cursor: usize,
    current_token: Token,
}

impl Context {
    /// create a context to parse a single expression takes the expression or an empty string that can be updated with
    ///  the set_text method
    pub fn new(program_text: &str) -> Context {
        Context {
            text: program_text.chars().collect(),
            cursor: 0,
            current_token: Token {
                token_type: TokenType::NOP,
                value: String::from("NOP"),
            },
        }
    }

    /// update the text of the expression
    pub fn set_text(&mut self, text: &str) {
        self.text = text.chars().collect();
    }

    fn get_next_token(&self) -> Token {
        let text = &self.text;
        if self.cursor > (text.len() - 1) {
            Token {
                token_type: TokenType::EOF,
                value: String::from("EOF"),
            }
        } else {
            match text[self.cursor] {
                c if c.is_digit(10) => Token {
                    token_type: TokenType::Int,
                    value: String::from(c.to_string()),
                },
                c if c.is_whitespace() => Token {
                    token_type: TokenType::SPC,
                    value: String::from("SPC"),
                },
                '+' => Token {
                    token_type: TokenType::Op,
                    value: String::from("+"),
                },
                '-' => Token {
                    token_type: TokenType::Op,
                    value: String::from("-"),
                },
                '/' => Token {
                    token_type: TokenType::Op,
                    value: String::from("/"),
                },
                '*' => Token {
                    token_type: TokenType::Op,
                    value: String::from("*"),
                },
                _ => panic!("parse error"),
            }
        }
    }

    fn eat(&mut self, expect_type: TokenType) {
        if self.current_token.token_type == expect_type {
            self.cursor += 1;
            self.current_token = self.get_next_token();
        } else {
            panic!("Illegal OP `{:?}` expected `{:?}` at pos {}:`{}`", self.current_token.token_type, expect_type, self.cursor, self.text[self.cursor])
        }
    }

    fn look_ahead(&mut self, by: usize) -> Token {
        self.cursor += by;
        let ahead = self.get_next_token();
        self.cursor -= by;
        ahead
    }

    fn eat_int(&mut self) -> i32 {
        let mut buffer = String::new();
        loop {
            if self.current_token.token_type == TokenType::Int {
                buffer.push_str(self.current_token.value.as_ref());
                self.eat(TokenType::Int);
            } else {
                break;
            }
        }
        buffer.parse::<i32>().unwrap()
    }

    fn eat_whitespace(&mut self) {
        if self.current_token.token_type == TokenType::SPC {
            self.eat(TokenType::SPC);
            if self.look_ahead(1).token_type == TokenType::SPC {
                self.eat_whitespace();
            }
        }
    }

    fn term(&mut self) -> i32 {
        self.eat_whitespace();
        self.eat_int()
    }

    fn expr(&mut self) -> i32 {

        self.current_token = self.get_next_token();

        let mut r = self.term();

        while self.current_token.token_type == TokenType::Op {
            let op = &self.current_token.value[..];
            let tk = self.current_token;
            self.eat(TokenType::Op);
            r  = match op {
                "+" => r + self.term(),
                "-" => r - self.term(),
                "/" => r / self.term(),
                "*" => r * self.term(),
                _ => panic!("illegal op `{}`", op),
            };
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term() {
        let mut ctx = Context::new("145+33");
        ctx.current_token = ctx.get_next_token();
        let r = ctx.term();
        
        assert_eq!(r, 145 as i32);
    }

    #[test]
    fn test_sum() {
        let mut ctx = Context::new("145 + 33");
        let r = ctx.expr();
        assert_eq!(r, 178 as i32);
    }

    #[test]
    fn test_sub() {
        let mut ctx = Context::new("178 - 33");
        let r = ctx.expr();
        assert_eq!(r, 145 as i32);
    }

    #[test]
    fn test_div() {
        let mut ctx = Context::new("44 / 4");
        let r = ctx.expr();
        assert_eq!(r, 11 as i32);
    }

    #[test]
    fn test_mul() {
        let mut ctx = Context::new("145 * 33");
        let r = ctx.expr();
        assert_eq!(r, 4785 as i32);
    }
}
