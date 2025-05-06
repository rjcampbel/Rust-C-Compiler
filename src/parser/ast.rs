use crate::lexer::token::Token;

#[macro_export]
macro_rules! expect_token {
   ($actual_token:expr, $expected_token:path, $msg:literal) => {
      if let Some(token) = $actual_token {
         match token {
            $expected_token => (),
            _ => {
               return Err(String::from($msg));
            }
         }
      } else {
         return Err(String::from($msg));
      }
   };
}

macro_rules! expect_assign_token {
   ($actual_token:expr, $expected_token:path, $var_name:ident, $type:ty, $msg:literal) => {
      let $var_name: $type;
      if let Some(token) = $actual_token {
         match token {
            $expected_token(value) => $var_name = value.clone(),
            _ => {
               return Err(String::from($msg));
            }
         }
      } else {
         return Err(String::from($msg));
      }
   };
}

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
      expect_token!(token_stream.next(), Token::Int, "Syntax Error: expected an int");
      expect_assign_token!(token_stream.next(), Token::Identifier, func_name, String, "Syntax Error");
      expect_token!(token_stream.next(), Token::OpenParen, "Syntax Error: expected open paren");
      expect_token!(token_stream.next(), Token::Void, "Syntax Error: expected void");
      expect_token!(token_stream.next(), Token::CloseParen, "Syntax Error: expected closing paren");
      expect_token!(token_stream.next(), Token::OpenBrace, "Syntax Error: expected open brace");
      let statement: Stmt = Stmt::parse(token_stream)?;
      expect_token!(token_stream.next(), Token::CloseBrace, "Syntax Error: expected closing brace");

      Ok(FuncDef::Function( Function { name: func_name, stmt: statement } ))
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
      expect_token!(token_stream.next(), Token::Return, "Syntax Error: expected return");
      let expression = Expr::parse(token_stream)?;
      expect_token!(token_stream.next(), Token::Semicolon, "Syntax Error: expected a semicolon");

      Ok(Stmt::Return(expression))
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
      expect_assign_token!(token_stream.next(), Token::Integer, number, u64, "Syntax Error: expected an integer");

      Ok(Expr::Const(number))
   }
}