use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
struct Node<'a> {
    token: Token<'a>,
    children: Option<Vec<Node<'a>>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Ast<'a> {
    root: Node<'a>,
}