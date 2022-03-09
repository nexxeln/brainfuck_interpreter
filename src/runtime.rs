use crate::Token;
use crate::parser::parse;
use read_input::prelude::*;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub code: Vec<Token>,
    pub code_pos: usize,
    pub memory: Vec<u8>,
    pub memory_pos: usize,
}

impl Runtime {
    pub fn new(parsed_code: &str) -> Runtime {
        Runtime {
            code: parse(parsed_code),
            code_pos: 0usize,
            memory: vec![0u8],
            memory_pos: 0usize,
        }
    }

    pub fn run(mut self) {
        loop {
            if self.code_pos < self.code.len() {
                match self.execute() {
                    Err(e) => {
                        println!("{}", e);
                        break;
                    }
                    Ok(v) => {
                        self = v.iterate().to_owned();
                    }
                };
            } else {
                break;
            }
        }
    }

    pub fn execute(&mut self) -> Result<&mut Runtime, &'static str> {
        let current_token = &self.code[self.code_pos];

        match current_token {
            Token::Inc(x) => {
                self.memory[self.memory_pos] =
                    ((self.memory[self.memory_pos] as usize + x) & 255) as u8;
                Ok(self)
            }
            Token::Dec(x) => {
                self.memory[self.memory_pos] =
                    ((self.memory[self.memory_pos] as isize - *x as isize) & 255) as u8;
                Ok(self)
            }
            Token::LMov(x) => {
                self.memory_pos += x;
                if self.memory_pos < self.memory.len() {
                    Ok(self)
                } else {
                    self.memory.append(&mut vec![0u8; *x]);
                    Ok(self)
                }
            }
            Token::RMov(x) => {
                let y = self.memory_pos.checked_sub(*x);
                match y {
                    None => Err("Error: Head moved off tape!\n"),
                    Some(v) => {
                        self.memory_pos = v;
                        Ok(self)
                    }
                }
            }
            Token::OutStd => {
                print!("{}", self.memory[self.memory_pos] as char);
                Ok(self)
            }
            Token::InStd => {
                self.memory[self.memory_pos] = input::<char>().msg("> ")
                    .add_err_test(|x| *x as usize <= 255, "Not an ascii value")
                    .get() as u8;
                Ok(self)
            }
            Token::OpenBrk(x) => {
                if self.memory[self.memory_pos] == 0 {
                    self.code_pos += x;
                    Ok(self)
                } else {
                    Ok(self)
                }
            }
            Token::ClosedBrk(x) => {
                if self.memory[self.memory_pos] != 0 {
                    self.code_pos -= x;
                    Ok(self)
                } else {
                    Ok(self)
                }
            }
        }
    }
    fn iterate(&mut self) -> &mut Runtime {
        self.code_pos += 1;
        self
    }
}