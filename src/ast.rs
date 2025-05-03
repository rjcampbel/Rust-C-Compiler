enum Program {
   Program(FuncDef),
}

struct Function {
   name: String,
   stmt: Stmt,
}

enum FuncDef {
   Function(Function),
}

enum Stmt {
   Return(Expr),
}

enum Expr {
   Const(u32),
}