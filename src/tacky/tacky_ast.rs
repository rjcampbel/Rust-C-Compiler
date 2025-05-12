use crate::parser::ast;

pub enum Program {
   Program(FuncDef),
}

impl Program {
   pub fn pretty_print(&self) {
      println!("Program(");
      match self {
         Self::Program(f) => {
            f.pretty_print(1);
         }
      }
      println!(")");
   }

   pub fn parse(ast: &ast::Program) -> Result<Program, String> {
      let func_def: FuncDef;
      match ast {
         ast::Program::Program(f) => {
            func_def = FuncDef::parse(f)?;
         }
      }
      Ok(Program::Program(func_def))
   }
}

pub enum FuncDef {
   Function(Function)
}

impl FuncDef {
   pub fn pretty_print(&self, indent_level: usize) {
      println!("{:indent$}Function(", "", indent=indent_level*3);
      match self {
         Self::Function(f) => {
            f.pretty_print(indent_level + 1);
         }
      }
      println!("{:indent$})", "", indent=indent_level*3);
   }

   pub fn parse(func_def: &ast::FuncDef) -> Result<FuncDef, String> {
      match func_def {
         ast::FuncDef::Function(f) => {
            let mut tacky_function = Function::new(&f.name);
            tacky_function.parse(f)?;
            Ok(FuncDef::Function(tacky_function))
         }
      }
   }
}

pub struct Function {
   pub identifier: String,
   pub instrs: Vec<Instr>,
   counter: usize
}

impl Function {
   pub fn new(name: &String) -> Self {
      Function {
         identifier: name.to_string(),
         instrs: Vec::new(),
         counter: 0
      }
   }

   pub fn pretty_print(&self, indent_level: usize) {
      println!("{:indent$}name={name}", "", indent=indent_level*3, name=self.identifier);
      println!("{:indent$}instrs=(size={size}) {instrs:?}", "", indent=indent_level*3, size=self.instrs.len(), instrs=self.instrs);
   }

   pub fn parse(&mut self, function: &ast::Function) -> Result<(), String> {
      match &function.stmt {
         ast::Stmt::Return(e) => {
            let ret = self.parse_expression(e)?;
            self.instrs.push(Instr::Return(ret));
         }
      }
      Ok(())
   }

   fn parse_expression(&mut self, expr: &ast::Expr) -> Result<Val, String> {
      match expr {
         ast::Expr::Const(c) => {
            return Ok(Val::Constant(*c));
         },
         ast::Expr::Unary(u) => {
            let inner = match &**u {
               ast::UnaryOp::Complement(e) => e,
               ast::UnaryOp::Negate(e) => e
            };
            let src: Val = self.parse_expression(inner)?;
            let dest_name: String = self.make_temporary();
            let dst = Val::Var(dest_name);
            let tacky_op =  match &**u {
               ast::UnaryOp::Complement(_) => UnaryOp::Complement,
               ast::UnaryOp::Negate(_) => UnaryOp::Negate
            };
            self.instrs.push(Instr::Unary(Unary { op:tacky_op, src, dst: dst.clone() }));
            return Ok(dst);
         },
         ast::Expr::Expr(e) => {
            return self.parse_expression(&**e);
         }
      }
   }

   fn make_temporary(&mut self) -> String {
      let tmp: String = format!("tmp.{}", self.counter);
      self.counter += 1;
      tmp
   }
}


#[derive(Debug)]
pub enum Instr {
   Return(Val),
   Unary(Unary)
}

#[derive(Debug)]
pub struct Unary {
   pub op: UnaryOp,
   pub src: Val,
   pub dst: Val
}

#[derive(Debug)]
pub enum UnaryOp {
   Complement,
   Negate,
}

#[derive(Debug,Clone)]
pub enum Val {
   Constant(u64),
   Var(String)
}
