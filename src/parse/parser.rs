use crate::ast::arith_bin_op_node::ArithmeticBinOpNode;
use crate::ast::exec_root_node::ExecRootNode;
use crate::ast::int_node::IntNode;
use crate::parse::parse_results::ParseResults;
use crate::parse::token::{Token, TokenType};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    tok_idx: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            tok_idx: 0
        }
    }

    pub fn parse(&mut self) -> ParseResults {
        println!("Parsing tokens: {:?}", self.tokens);

        let mut res = ParseResults::new();

        if self.tokens.len() == 0 {
            return res.success(Box::new(ExecRootNode::new(None)));
        }

        let expr = res.register(self.expression());
        if res.error.is_some() {
            return res;
        }
        let root_node = ExecRootNode::new(expr);

        if self.tok_idx < self.tokens.len() {
            panic!("Unexpected token: {:?}", self.tokens[self.tok_idx]);
        }

        res.success(Box::new(root_node))
    }

    fn advance(&mut self, res: Option<&mut ParseResults>) -> Token {
        if self.tok_idx >= self.tokens.len() {
            panic!("Unexpected end of input at token {}", self.tok_idx);
        }

        if let Some(res) = res {
            res.register_advancement();
        }

        self.tok_idx += 1;
        self.tokens[self.tok_idx-1].clone()
    }

    fn try_advance(&mut self) -> Option<Token> {
        if self.tok_idx >= self.tokens.len() {
            return None;
        }
        self.tok_idx += 1;
        Some(self.tokens[self.tok_idx-1].clone())
    }

    fn try_peak(&self) -> Option<Token> {
        if self.tok_idx >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.tok_idx].clone())
    }

    fn expression(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let lhs = res.register(self.atom());
        if res.error.is_some() {
            return res;
        }

        let operand = self.try_peak();

        if let Some(op) = operand {
            if op.token_type == TokenType::Plus || op.token_type == TokenType::Sub {
                self.advance(Some(&mut res));

                let rhs = res.register(self.expression());
                if res.error.is_some() {
                    return res;
                }

                return res.success(Box::new(ArithmeticBinOpNode::new(
                    lhs.unwrap(),
                    (if op.token_type == TokenType::Plus { "add"  } else { "sub" }).to_owned(),
                    rhs.unwrap()
                )))
            }
        }

        ParseResults::from_node(lhs.unwrap())
    }

    fn atom(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let tok = self.advance(Some(&mut res));
        res.success(match tok.token_type {
            TokenType::Int => {
                let value = tok.literal.unwrap();
                Box::new(IntNode::new(value.parse::<i64>().unwrap()))
            }
            _ => panic!("Unexpected token: {:?}", tok)
        })
    }
}