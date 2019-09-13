mod ast;
mod parser;

use std::collections::HashMap;

use ast::{Node, Op};
use parser::Parser;
use rustyline::Editor;

struct Context {
    precision: Option<usize>,
    vars: HashMap<String, Node>,
}

fn apply_op(op: Op, a: f64, b: f64) -> f64 {
    match op {
        Op::Add => a + b,
        Op::Subtract => a - b,
        Op::Multiply => a * b,
        Op::Divide => a / b,
        Op::Power => a.powf(b),
    }
}

fn apply_fun(name: &String, arg: f64) -> Option<f64> {
    match name.as_str() {
        "sin" => Some(arg.sin()),
        "cos" => Some(arg.cos()),
        "tan" => Some(arg.tan()),
        _ => None,
    }
}

fn eval(node: &Node, ctx: &Context) -> Option<f64> {
    match node {
        Node::Number(x) => Some(*x),
        Node::Variable(x) => ctx.vars.get(x).and_then(|n| eval(n, ctx)),
        Node::Function { name, arg } => {
            let x = eval(arg, ctx)?;
            apply_fun(name, x)
        }
        Node::Assign {..} => None,
        Node::Op { op, lhs, rhs } => {
            let x = eval(lhs, ctx)?;
            let y = eval(rhs, ctx)?;
            Some(apply_op(*op, x, y))
        }
    }
}

fn _reduce(node: &Node, ctx: &Context) -> (Box<Node>, bool) {
    match node {
        Node::Number(_) => (Box::new(node.clone()), false),
        Node::Function { name, arg } => {
            let (newarg, changed) = _reduce(arg, ctx);
            match *newarg {
                Node::Number(x) => match apply_fun(name, x) {
                    Some(y) => (Box::new(Node::Number(y)), true),
                    None => (Box::new(Node::Function { name: name.clone(), arg: Box::new(Node::Number(x)) }), true),
                }
                _ => (Box::new(Node::Function { name: name.clone(), arg: newarg.clone() }), changed)
            }
        },
        Node::Variable(x) => {
            match ctx.vars.get(x).and_then(|n| eval(n, ctx)) {
                Some(y) => (Box::new(Node::Number(y)), true),
                None => (Box::new(node.clone()), false),
            }
        }
        Node::Assign { name, rhs } => {
            let (newrhs, changed) = _reduce(rhs, ctx);
            (Box::new(Node::Assign { name: name.clone(), rhs: newrhs }), changed)
        }
        Node::Op { op, lhs, rhs } => {
            let left: &Node = lhs;
            let right: &Node = rhs;
            let (new, changed) = match (left, right) {
                (Node::Number(a), Node::Number(b)) => (Node::Number(apply_op(*op, *a, *b)), true),
                (Node::Variable(a), Node::Number(b)) => {
                    match ctx.vars.get(a).and_then(|n| eval(n, ctx)) {
                        Some(x) => (Node::Number(apply_op(*op, x, *b)), true),
                        None => (node.clone(), false),
                    }
                }
                (Node::Number(a), Node::Variable(b)) => {
                    match ctx.vars.get(b).and_then(|n| eval(n, ctx)) {
                        Some(y) => (Node::Number(apply_op(*op, *a, y)), true),
                        None => (node.clone(), false),
                    }
                }
                (Node::Variable(a), Node::Variable(b)) => {
                    match (ctx.vars.get(a).and_then(|n| eval(n, ctx)), ctx.vars.get(b).and_then(|n| eval(n, ctx))) {
                        (Some(x), Some(y)) => (Node::Number(apply_op(*op, x, y)), true),
                        _ => (node.clone(), false),
                    }
                }
                (a, b) => {
                    let (newlhs, changed_a) = _reduce(a, ctx);
                    let (newrhs, changed_b) = _reduce(b, ctx);
                    (Node::Op { op: *op, lhs: newlhs, rhs: newrhs }, changed_a || changed_b)
                }
            };
            (Box::new(new), changed)
        }
    }
}

fn reduce(node: &Node, ctx: &Context) -> Box<Node> {
    let (new, changed) = _reduce(node, ctx);
    let mut n = new;
    let mut c = changed;
    while c {
        let (new, changed) = _reduce(&n, ctx);
        n = new;
        c = changed;
    }
    n
}

fn update_context(node: &Node, ctx: &mut Context) {
    match node {
        Node::Assign { name, rhs } => {
            if name == "precision" {
                match eval(rhs, ctx) {
                    Some(x) => ctx.precision = Some(x as usize),
                    None => println!("Oops, could not evaluate precision!"),
                }
            } else {
                ctx.vars.insert(name.clone(), *rhs.clone());
            }
        }
        _ => (),
    }
}

fn main() {
    println!("Recalc v0.1");

    let p = Parser::new();
    let mut rl = Editor::<()>::new();
    let mut ctx = Context {
        precision: None,
        vars: HashMap::new(),
    };

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match p.parse_string(&line) {
                    Ok(node) => {
                        update_context(&node, &mut ctx);
                        match ctx.precision {
                            None => println!("{}", reduce(&node, &ctx)),
                            Some(p) => println!("{1:.*}", p, reduce(&node, &ctx)),
                        }
                    }
                    Err(err) => println!("{}", err),
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }
}
