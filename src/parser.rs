use crate::codegen::CodeGen;
/// Parser for Retro BASIC using tokens
/// Parses tokens and generates B-code
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    codegen: CodeGen,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            codegen: CodeGen::new(),
        }
    }

    pub fn parse(&mut self) -> Result<String, String> {
        self.parse_program()?;
        Ok(self.codegen.format_output())
    }

    // Helper methods
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current()))
        }
    }

    // Main parsing methods
    fn parse_program(&mut self) -> Result<(), String> {
        while self.current().is_some() {
            self.parse_line()?;
        }
        Ok(())
    }

    fn parse_line(&mut self) -> Result<(), String> {
        let line_num = match self.current() {
            Some(Token::LineNum(n)) => {
                let num = *n;
                self.advance();
                num
            }
            _ => return Err(format!("Expected line number, got {:?}", self.current())),
        };

        self.codegen.emit_line(line_num);
        self.parse_statement()?;
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<(), String> {
        match self.current() {
            Some(Token::Id(_)) => self.parse_assignment(),
            Some(Token::If) => self.parse_if(),
            Some(Token::Print) => self.parse_print(),
            Some(Token::Goto) => self.parse_goto(),
            Some(Token::Stop) => self.parse_stop(),
            _ => Err(format!(
                "Unexpected token in statement: {:?}",
                self.current()
            )),
        }
    }

    fn parse_assignment(&mut self) -> Result<(), String> {
        let id = match self.current() {
            Some(Token::Id(c)) => {
                let c = *c;
                self.advance();
                c
            }
            _ => return Err(format!("Expected identifier, got {:?}", self.current())),
        };

        self.expect(Token::Assign)?;
        self.codegen.emit_id(id);
        self.codegen.emit_op('=');
        self.parse_expression()?;
        Ok(())
    }

    fn parse_expression(&mut self) -> Result<(), String> {
        self.parse_term()?;

        while matches!(self.current(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = match self.current() {
                Some(Token::Plus) => {
                    self.advance();
                    '+'
                }
                Some(Token::Minus) => {
                    self.advance();
                    '-'
                }
                _ => unreachable!(),
            };

            self.codegen.emit_op(op);
            self.parse_term()?;
        }

        Ok(())
    }

    fn parse_term(&mut self) -> Result<(), String> {
        match self.current() {
            Some(Token::Id(c)) => {
                let c = *c;
                self.advance();
                self.codegen.emit_id(c);
                Ok(())
            }
            Some(Token::LineNum(n)) => {
                let n = *n;
                self.advance();
                self.codegen.emit_const(n);
                Ok(())
            }
            _ => Err(format!(
                "Expected term (id or number), got {:?}",
                self.current()
            )),
        }
    }

    fn parse_if(&mut self) -> Result<(), String> {
        self.expect(Token::If)?;
        self.codegen.emit_if();
        self.parse_condition()?;

        let target = match self.current() {
            Some(Token::LineNum(n)) => {
                let num = *n;
                self.advance();
                num
            }
            _ => {
                return Err(format!(
                    "Expected line number after condition, got {:?}",
                    self.current()
                ))
            }
        };

        self.codegen.emit_goto(target);
        Ok(())
    }

    fn parse_condition(&mut self) -> Result<(), String> {
        self.parse_condition_term()?;

        match self.current() {
            Some(Token::Less) => {
                self.advance();
                self.codegen.emit_op('<');
                self.parse_condition_term()?;
                Ok(())
            }
            Some(Token::Assign) => {
                self.advance();
                self.codegen.emit_op('=');
                self.parse_condition_term()?;
                Ok(())
            }
            _ => Err(format!(
                "Expected comparison operator, got {:?}",
                self.current()
            )),
        }
    }

    fn parse_condition_term(&mut self) -> Result<(), String> {
        match self.current() {
            Some(Token::Id(c)) => {
                let c = *c;
                self.advance();
                self.codegen.emit_id(c);
                Ok(())
            }
            Some(Token::LineNum(n)) => {
                let n = *n;
                self.advance();
                self.codegen.emit_const(n);
                Ok(())
            }
            _ => Err(format!("Expected condition term, got {:?}", self.current())),
        }
    }

    fn parse_print(&mut self) -> Result<(), String> {
        self.expect(Token::Print)?;
        self.codegen.emit_print();

        let id = match self.current() {
            Some(Token::Id(c)) => {
                let c = *c;
                self.advance();
                c
            }
            _ => {
                return Err(format!(
                    "Expected identifier after PRINT, got {:?}",
                    self.current()
                ))
            }
        };

        self.codegen.emit_id(id);
        Ok(())
    }

    fn parse_goto(&mut self) -> Result<(), String> {
        self.expect(Token::Goto)?;

        let target = match self.current() {
            Some(Token::LineNum(n)) => {
                let num = *n;
                self.advance();
                num
            }
            _ => {
                return Err(format!(
                    "Expected line number after GOTO, got {:?}",
                    self.current()
                ))
            }
        };

        self.codegen.emit_goto(target);
        Ok(())
    }

    fn parse_stop(&mut self) -> Result<(), String> {
        self.expect(Token::Stop)?;
        self.codegen.emit_stop();
        Ok(())
    }
}
