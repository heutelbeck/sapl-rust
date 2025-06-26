trait Expression: Eval {}
trait Eval {
    fn eval(&self) -> f64;
}

struct ConstantExpr {
    v: f64,
}

impl Expression for ConstantExpr {}
impl Eval for ConstantExpr {
    fn eval(&self) -> f64 {
        self.v
    }
}

struct AddExpr {
    l: Box<dyn Expression>,
    r: Box<dyn Expression>,
}

impl Expression for AddExpr {}
impl Eval for AddExpr {
    fn eval(&self) -> f64 {
        self.l.eval() + self.r.eval()
    }
}

struct MulExpr {
    l: Box<dyn Expression>,
    r: Box<dyn Expression>,
}

impl Expression for MulExpr {}
impl Eval for MulExpr {
    fn eval(&self) -> f64 {
        self.l.eval() * self.r.eval()
    }
}

fn interpret<Expr: Expression>(expr: Expr) -> f64 {
    expr.eval()
}

fn main() {
    let expression = MulExpr {
        l: Box::new(AddExpr {
            l: Box::new(ConstantExpr { v: 5.0 }),
            r: Box::new(ConstantExpr { v: 10.0 }),
        }),
        r: Box::new(ConstantExpr { v: 2.0 }),
    };

    println!("result: {}", interpret(expression));
}
