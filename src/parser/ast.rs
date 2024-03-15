/*
 * @author: ruka-lang
 * @created: 2024-02-28
 */

use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Binding(Binding),
    Return(Expression),
    Expression(Expression)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Binding {
    pub kind: Keyword,
    pub name: Arc<str>,
    pub expl_type: Option<Type>,
    pub value: Expression
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Unit,
    Tag(Arc<str>),
    Integer(Box<str>),
    Float(Box<str>),
    Boolean(bool),
    Block(Block),
    If(Box<If>),
    Match(Box<Match>),
    Fn(Box<Fn>),
    Closure(Box<Closure>),
    FnCall(Box<FnCall>),
    Prefix(Box<Prefix>),
    Infix(Box<Infix>),
    Postfix(Box<Postfix>)
}

#[derive(Debug, PartialEq, Eq)]
pub struct If {
    pub condition: Expression,
    pub consequence: Expression,
    pub alternative: Expression
}

#[derive(Debug, PartialEq, Eq)]
pub struct Match {
    pub value: Expression,
    pub cases: Vec<Case>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Case {
    pub condition: Expression,
    pub consequence: Expression
}

#[derive(Debug, PartialEq, Eq)]
pub struct Type {
    
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fn {
    pub name: Arc<str>,
    pub parameters: Vec<Arc<str>>,
    pub block: Expression,
    pub arity: usize
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Node>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Closure {
    pub name: Arc<str>,
    pub parameters: Vec<Arc<str>>,
    pub block: Expression,
    pub context: Vec<Arc<str>>,
    pub arity: usize
}

#[derive(Debug, PartialEq, Eq)]
pub struct FnCall {
    pub func: Expression,
    pub args: Vec<Expression>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Prefix {
    pub operator: Kind,
    pub value: Expression
}

#[derive(Debug, PartialEq, Eq)]
pub struct Infix {
    pub operator: Kind,
    pub lhs: Expression,
    pub rhs: Expression
}

#[derive(Debug, PartialEq, Eq)]
pub struct Postfix {
    pub operator: Kind,
    pub value: Expression
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ast {
    pub nodes: Vec<Node>
}
