use crate::lexer::token::Token;

pub enum Program {
   Program(FuncDef),
}

impl Program {
   pub fn pretty_print(&self) {
      println!("Program");
      match self {
         Self::Program(f) => f.pretty_print(),
      }
   }

   pub fn parse(tokens: &Vec<Token>) -> Result<Self, String> {
      let mut token_stream = tokens.iter();
      let parse_result = FuncDef::parse(&mut token_stream)?;

      if let Some(_) = token_stream.next() {
         return Err(String::from("Unexpected junk found"));
      }
      Ok(Program::Program(parse_result))
   }
}

pub struct Function {
   name: String,
   stmt: Stmt,
}

pub enum FuncDef {
   Function(Function),
}

impl FuncDef {
   pub fn pretty_print(&self) {
      match self {
         Self::Function(f) => {
            println!("Function name = {}", f.name);
            f.stmt.pretty_print();
         }
      }
   }

   pub fn parse<'a>(token_stream: &mut impl Iterator<Item=&'a Token>) -> Result<Self, String> {
      if let Some(token) = token_stream.next() {
         match token {
            Token::Int => (),
            _ => {
               return Err(String::from("Syntax Error: expected int"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected an int"));
      }

      let mut func_name = String::new();
      if let Some(token) = token_stream.next() {
         match token {
            Token::Identifier(name) => func_name = name.to_string(),
            _ => {
               return Err(String::from("Syntax Error: expected identifier"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected an identifier"));
      }

      if let Some(token) = token_stream.next() {
         match token {
            Token::OpenParen => (),
            _ => {
               return Err(String::from("Syntax Error: expected open paren"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected an open paren"));
      }

      if let Some(token) = token_stream.next() {
         match token {
            Token::Void => (),
            _ => {
               return Err(String::from("Syntax Error: expected void"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected void"));
      }

      if let Some(token) = token_stream.next() {
         match token {
            Token::CloseParen => (),
            _ => {
               return Err(String::from("Syntax Error: expected closing paren"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected a closing paren"));
      }

      if let Some(token) = token_stream.next() {
         match token {
            Token::OpenBrace => (),
            _ => {
               return Err(String::from("Syntax Error: expected open brace"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected an open brace"));
      }

      let parse_result: Stmt = Stmt::parse(token_stream)?;

      if let Some(token) = token_stream.next() {
         match token {
            Token::CloseBrace => (),
            _ => {
               return Err(String::from("Syntax Error: expected closing brace"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected a closing brace"));
      }

      Ok(FuncDef::Function( Function { name: func_name, stmt: parse_result } ))
   }
}

pub enum Stmt {
   Return(Expr)
}

impl Stmt {
   pub fn pretty_print(&self) {
      match self {
         Self::Return(e) => {
            println!("Return");
            e.pretty_print();
         }
      }
   }

   pub fn parse<'a>(token_stream: &mut impl Iterator<Item=&'a Token>) -> Result<Self, String> {
      if let Some(token) = token_stream.next() {
         match token {
            Token::Return => (),
            _ => {
               return Err(String::from("Syntax Error: expected a return"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected a return"));
      }

      let parse_result = Expr::parse(token_stream)?;
      if let Some(token) = token_stream.next() {
         match token {
            Token::Semicolon => (),
            _ => {
               return Err(String::from("Syntax Error: expected a semicolon"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected a semicolon"));
      }
      // expect semicolon
      Ok(Stmt::Return(parse_result))
   }
}

pub enum Expr {
   Const(u64)
}

impl Expr {
   pub fn pretty_print(&self) {
      match self {
         Self::Const(c) => {
            println!("Constant: {}", c);
         }
      }
   }

   pub fn parse<'a>(token_stream: &mut impl Iterator<Item=&'a Token>) -> Result<Self, String> {
      let mut value: u64 = 0;
      if let Some(token) = token_stream.next() {
         match token {
            Token::Integer(i) => value = *i,
            _ => {
               return Err(String::from("Syntax Error: expected an integer"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected an integer"));
      }

      Ok(Expr::Const(value))
   }
}