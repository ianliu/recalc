use pest_derive::Parser;
use crate::ast::{Node, Op};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser as PestParser;

#[derive(Parser)]
#[grammar = "recalc.pest"]
struct RecalcParser;

pub struct Parser {
    prec: PrecClimber<Rule>,
}

impl Parser {
    pub fn new() -> Self {
        use Assoc::*;
        use Rule::*;

        Parser {
            prec: PrecClimber::new(vec![
                Operator::new(add, Left) | Operator::new(subtract, Left),
                Operator::new(multiply, Left) | Operator::new(divide, Left),
                Operator::new(power, Right),
            ]),
        }
    }

    pub fn parse(&self, expression: Pairs<Rule>) -> Node {
        self.prec.climb(
            expression,
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::num => Node::Number(pair.as_str().parse::<f64>().unwrap()),
                Rule::assign => {
                    let mut iterator = pair.into_inner();
                    let name = iterator.next().unwrap().as_str().to_string();
                    let rhs = Box::new(self.parse(iterator));
                    Node::Assign { name, rhs }
                }
                Rule::pi => Node::Number(std::f64::consts::PI),
                Rule::e => Node::Number(std::f64::consts::E),
                Rule::ident => Node::Variable(pair.as_str().to_string()),
                Rule::expr => self.parse(pair.into_inner()),
                Rule::function => {
                    let mut iterator = pair.into_inner();
                    let name = iterator.next().unwrap().as_str().to_string();
                    let arg = Box::new(self.parse(iterator));
                    Node::Function { name, arg }
                }
                _ => unreachable!(),
            },
            |lhs: Node, op: Pair<Rule>, rhs: Node| match op.as_rule() {
                Rule::add => Node::Op {
                    op: Op::Add,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::subtract => Node::Op {
                    op: Op::Subtract,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::multiply => Node::Op {
                    op: Op::Multiply,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::divide => Node::Op {
                    op: Op::Divide,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Rule::power => Node::Op {
                    op: Op::Power,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                _ => unreachable!(),
            },
        )
    }

    pub fn parse_string(&self, text: &str) -> Result<Node, Error<Rule>> {
        RecalcParser::parse(Rule::calculation, &text).map(|calc| self.parse(calc))
    }
}
