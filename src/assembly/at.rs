use crate::parser::ast;

pub enum Program {
   Program(FuncDef)
}

impl Program {
   pub fn pretty_print(&self) {
      println!("Program(");
      match self {
         Program::Program(f) => {
            f.pretty_print(1);
         }
      }
      println!(")");
   }

   pub fn parse(program: &ast::Program) -> Result<Self, String> {
      let function_def: FuncDef;
      match program {
         ast::Program::Program(func_def) => {
            function_def = FuncDef::parse(func_def)?;
         }
      }
      Ok(Program::Program(function_def))
   }
}

pub struct Function {
   name: String,
   instrs: Vec<Inst>
}

impl Function {
   pub fn new(name_: &String) -> Self {
      Function {
         name: name_.to_string(),
         instrs: Vec::new()
      }
   }

   pub fn pretty_print(&self, indent_level: usize) {
      println!("{:indent$}Name({name})", "", indent=indent_level*3, name=self.name);
      println!("{:indent$}Instrs(", "", indent=indent_level*3);
      for instr in &self.instrs {
         instr.pretty_print(indent_level+1);
      }
   }

   pub fn parse(function: &ast::Function) -> Result<Self, String> {
      let mut at_func: Function = Function::new(&function.name);

      match &function.stmt {
         ast::Stmt::Return(expr) => {
            match expr {
               ast::Expr::Const(c) => {
                  at_func.instrs.push(Inst::Mov(Mov{src:Operand::Imm(*c), dst:Operand::Register}));
                  at_func.instrs.push(Inst::Ret);
               }
            }
         }
      }

      Ok(at_func)
   }
}

pub enum FuncDef {
   Function(Function)
}

impl FuncDef {
   pub fn pretty_print(&self, indent_level: usize) {
      println!("{:indent$}Function(", "", indent=indent_level*3);
      match self {
         FuncDef::Function(f) => f.pretty_print(indent_level+1),
      }
      println!("{:indent$})", "", indent=indent_level*3);
   }

   pub fn parse(func_def: &ast::FuncDef) -> Result<Self, String> {
      let at_func: Function;
      match func_def {
         ast::FuncDef::Function(function) => {
            at_func = Function::parse(&function)?;
         }
      }
      Ok(FuncDef::Function(at_func))
   }
}

pub struct Mov {
   src: Operand,
   dst: Operand
}

impl Mov {
   pub fn pretty_print(&self, indent_level: usize) {
      println!("{:indent$}Src(", "", indent=indent_level*3);
      self.src.pretty_print(indent_level+1);
      println!("{:indent$})", "", indent=indent_level*3);
      println!("{:indent$}Dst(", "", indent=indent_level*3);
      self.dst.pretty_print(indent_level+1);
      println!("{:indent$})", "", indent=indent_level*3);
   }
}

pub enum Inst {
   Mov(Mov),
   Ret
}

impl Inst {
   pub fn pretty_print(&self, indent_level: usize) {
      match self {
         Inst::Mov(m) => {
            println!("{:indent$}Mov(", "", indent=indent_level*3);
            m.pretty_print(indent_level+1);
         },
         Inst::Ret => {
            println!("{:indent$}Ret", "", indent=indent_level*3);
         }
      }
   }
}

pub enum Operand {
   Imm(u64),
   Register
}

impl Operand {
   pub fn pretty_print(&self, indent_level: usize) {
      match self {
         Operand::Imm(v) => {
            println!("{:indent$}Imm({v})", "", indent=indent_level*3, v=v);
         },
         Operand::Register => {
            println!("{:indent$}Register", "", indent=indent_level*3);
         }
      }
   }
}