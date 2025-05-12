use std::fs;
use std::io::Write;

use crate::tacky::tacky_ast;

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

   pub fn parse(program: &tacky_ast::Program) -> Result<Self, String> {
      let function_def: FuncDef;
      match program {
         tacky_ast::Program::Program(func_def) => {
            function_def = FuncDef::parse(func_def)?;
         }
      }
      Ok(Program::Program(function_def))
   }

   pub fn write(&self, text: &mut fs::File) -> std::io::Result<()> {
      match self {
         Program::Program(p) => {
            p.write(text)?;
         }
      }
      Ok(())
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

   pub fn write(&self, text: &mut fs::File) -> std::io::Result<()> {
      writeln!(text, "   .globl _{}", self.name)?;
      writeln!(text, "_{}:", self.name)?;
      for instr in &self.instrs {
         instr.write(text)?;
      };
      Ok(())
   }

   pub fn parse(function: &tacky_ast::Function) -> Result<Self, String> {
      let mut at_func: Function = Function::new(&function.identifier);

      for instr in &function.instrs {
         match instr {
            tacky_ast::Instr::Return(v) => {
               let operand = match &v {
                  tacky_ast::Val::Constant(c) => {
                     Operand::Imm(*c)
                  },
                  tacky_ast::Val::Var(var) => {
                     Operand::PseudoReg(var.to_string())
                  }
               };
               at_func.instrs.push(Inst::Mov(Mov{src:operand, dst:Operand::Register(Reg::AX)}));
               at_func.instrs.push(Inst::Ret);
            },
            tacky_ast::Instr::Unary(op ) => {
               let src = match &op.src {
                  tacky_ast::Val::Constant(c) => {
                     Operand::Imm(*c)
                  },
                  tacky_ast::Val::Var(v) => {
                     Operand::PseudoReg(v.to_string())
                  },
               };
               let dst = match &op.dst {
                  tacky_ast::Val::Constant(c) => {
                     Operand::Imm(*c)
                  },
                  tacky_ast::Val::Var(v) => {
                     Operand::PseudoReg(v.to_string())
                  }
               };
               let op = match op.op {
                  tacky_ast::UnaryOp::Complement => UnaryOp::Complement,
                  tacky_ast::UnaryOp::Negate => UnaryOp::Negate
               };
               at_func.instrs.push(Inst::Mov(Mov{src, dst:dst.clone()}));
               at_func.instrs.push(Inst::Unary(op, dst.clone()));
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

   pub fn parse(func_def: &tacky_ast::FuncDef) -> Result<Self, String> {
      let at_func: Function;
      match func_def {
         tacky_ast::FuncDef::Function(function) => {
            at_func = Function::parse(&function)?;
         }
      }
      Ok(FuncDef::Function(at_func))
   }

   pub fn write(&self, text: &mut fs::File) -> std::io::Result<()> {
      match self {
         FuncDef::Function(f) => {
            f.write(text)?;
         }
      }
      Ok(())
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
   Unary(UnaryOp, Operand),
   AllocStack(u64),
   Ret
}

pub enum UnaryOp {
   Negate,
   Complement,
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
         },
         _ => ()
      }
   }

   pub fn write(&self, text: &mut fs::File) -> std::io::Result<()> {
      match self {
         Inst::Mov(m) => {
            write!(text, "   movl ")?;
            m.src.write(text)?;
            write!(text, ", ")?;
            m.dst.write(text)?;
            writeln!(text)?
         },
         Inst::Ret => {
            writeln!(text, "   ret")?;
         },
         _ => ()
      }
      Ok(())
   }
}

#[derive(Clone)]
pub enum Operand {
   Imm(u64),
   Register(Reg),
   PseudoReg(String),
   Stack(u64)
}

impl Operand {
   pub fn pretty_print(&self, indent_level: usize) {
      match self {
         Operand::Imm(v) => {
            println!("{:indent$}Imm({v})", "", indent=indent_level*3, v=v);
         },
         Operand::Register(r) => {
            let reg_name = match r {
               Reg::AX => "AX",
               Reg::R10 => "R10"
            };
            println!("{:indent$}Register({reg})", "", indent=indent_level*3, reg=reg_name);
         },
         Operand::PseudoReg(n) => {
            println!("{:indent$}PseudoReg({reg})", "", indent=indent_level*3, reg=n);
         },
         Operand::Stack(s) => {
            println!("{:indent$}Stack({bytes})", "", indent=indent_level*3, bytes=s);
         }
      }
   }

   pub fn write(&self, text: &mut fs::File) -> std::io::Result<()> {
      match self {
         Operand::Imm(v) => {
            write!(text, "${}", v)?;
         },
         Operand::Register(r) => {
            let reg_name = match r {
               Reg::AX => "%eax",
               Reg::R10 => "%r10d",
            };
            write!(text, "{}", reg_name)?;
         },
         Operand::PseudoReg(_) => (),
         Operand::Stack(_) => (),
      }

      Ok(())
   }
}

#[derive(Clone)]
pub enum Reg {
   AX,
   R10,
}