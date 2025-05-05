use crate::lexer::token::Token;

pub enum Program {
   Program(FuncDef),
}

impl Program {
   pub fn pretty_print(&self) {
      println!("Program(");
      match self {
         Self::Program(f) => f.pretty_print(1),
      }
      println!(")")
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
   pub fn pretty_print(&self, indent_level: usize) {
      match self {
         Self::Function(f) => {
            println!("{:indent$}Function(", "", indent=indent_level*3);
            println!("{:indent$}name=\"{name}\"", "", indent=indent_level*6, name=f.name);
            println!("{:indent$}body=(", "", indent=indent_level*6);
            f.stmt.pretty_print(indent_level+2);
            println!("{:indent$})", "", indent=indent_level*6);
            println!("{:indent$})", "", indent=indent_level*3);
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

      let func_name: String;
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
   pub fn pretty_print(&self, indent_level: usize) {
      match self {
         Self::Return(e) => {
            println!("{:indent$}Return(", "", indent=indent_level*3);
            e.pretty_print(indent_level+1);
            println!("{:indent$})", "", indent=indent_level*3);
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

      Ok(Stmt::Return(parse_result))
   }
}

pub enum Expr {
   Const(u64)
}

impl Expr {
   pub fn pretty_print(&self, indent_level: usize) {
      match self {
         Self::Const(c) => {
            println!("{:indent$}Constant({c})", "", indent=indent_level*3, c=c);
         }
      }
   }

   pub fn parse<'a>(token_stream: &mut impl Iterator<Item=&'a Token>) -> Result<Self, String> {
      if let Some(token) = token_stream.next() {
         match token {
            Token::Integer(i) => {
               return Ok(Expr::Const(*i))
            }
            _ => {
               return Err(String::from("Syntax Error: expected an integer"));
            }
         }
      } else {
         return Err(String::from("Syntax Error: expected an integer"));
      }

   }
}