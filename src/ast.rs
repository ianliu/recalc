use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Op::Add => "+",
            Op::Subtract => "-",
            Op::Multiply => "·",
            Op::Divide => "÷",
            Op::Power => "^",
        })
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Number(f64),
    Variable(String),
    Function {
        name: String,
        arg: Box<Node>,
    },
    Op {
        op: Op,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Assign {
        name: String,
        rhs: Box<Node>,
    },
}

fn op_precedence(op: &Op) -> i32 {
    match op {
        Op::Add | Op::Subtract    => 1,
        Op::Multiply | Op::Divide => 2,
        Op::Power                 => 3,
    }
}

fn format_branch(op: &Op, node: &Node) -> String {
    match node {
        Node::Number(_) => node.to_string(),
        Node::Variable(_) => node.to_string(),
        Node::Assign {..} => node.to_string(),
        Node::Function {..} => node.to_string(),
        Node::Op { op: op2, .. } => {
            if op_precedence(op) > op_precedence(op2) {
                format!("({})", node.to_string())
            } else {
                node.to_string()
            }
        },
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Node::Number(x) => {
                match f.precision() {
                    Some(p) => format!("{1:.*}", p, x),
                    None => x.to_string(),
                }
            },
            Node::Variable(x) => x.clone(),
            Node::Function { name, arg } => format!("{}({})", name, arg),
            Node::Op { op, lhs, rhs } => {
                format!("{} {} {}",
                        format_branch(op, lhs),
                        op.to_string(),
                        format_branch(op, rhs))
            },
            Node::Assign { name, rhs } => format!("{} = {}", name, rhs.to_string()),
        };
        write!(f, "{}", s)
    }
}
