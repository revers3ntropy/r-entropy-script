use std::collections::HashMap;
use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Context, SymbolDec, SymbolDef};
use crate::error::{Error, type_error_unstructured};

#[derive(Debug)]
pub struct FnDeclarationNode {
    pub identifier: String,
    pub ret_type: Box<dyn Node>,
    pub params: HashMap<String, Box<dyn Node>>,
    pub body: Option<Box<dyn Node>>
}

impl Node for FnDeclarationNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        if let Some(mut body_node) = self.body.take() {
            println!("body node: {:?}", body_node);
            let body = body_node.asm(ctx)?;
            ctx.define(SymbolDef {
                name: self.identifier.clone(),
                is_local: false,
                data: None,
                text: Some(format!("
                    push rbp
                    mov rbp, rsp
                    {body}
                    mov rsp, rbp
                    pop rbp
                    ret
                 "))
            }, false)?;
        }
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        if !ctx.allow_overrides && ctx.has_dec_with_id(self.identifier.clone().as_str()) {
            return Err(type_error_unstructured(format!("Symbol {} is already defined", self.identifier)))
        }
        let ret_type = self.ret_type.type_check(ctx);

        let mut children = vec![ret_type?];
        for param in self.params.values_mut() {
            children.push(param.type_check(ctx)?);
        }

        let this_type = Type {
            id: ctx.get_type_id(),
            name: "Fn".to_owned(),
            children
        };
        if self.body.is_some() {
            ctx.declare(SymbolDec {
                name: self.identifier.clone(),
                is_constant: true,
                is_type: false,
                type_: Box::new(this_type.clone())
            })?;
        } else {
            ctx.declare(SymbolDec {
                name: self.identifier.clone(),
                is_constant: true,
                is_type: false,
                type_: Box::new(this_type.clone())
            })?;
        }

        Ok(Box::new(this_type))
    }
}
