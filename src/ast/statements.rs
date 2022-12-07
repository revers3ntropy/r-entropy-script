use crate::ast::{Node, TypeCheckRes};
use crate::context::{Ctx};
use crate::error::{Error, type_error};

#[derive(Debug)]
pub struct StatementsNode {
    pub statements: Vec<Box<dyn Node>>,
    pub src: Vec<String>
}

impl Node for StatementsNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let mut asm = String::new();

        let mut i = 0;
        for statement in self.statements.iter_mut() {
            let stmt = statement.asm(ctx.clone())?;
            if !stmt.is_empty() {
                asm.push_str("\n;- SRC: ");
                asm.push_str(self.src.get(i).unwrap());
                asm.push_str("\n");
                asm.push_str(&stmt.clone());
            }
            i += 1;
        }
        Ok(asm)
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        let mut ret_types = Vec::new();
        for statement in self.statements.iter_mut() {
            let t = statement.type_check(ctx.clone())?;
            if t.1.is_some() {
                ret_types.push(t.1.unwrap())
            }
        }
        if ret_types.len() < 1 {
            return Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), None));
        }

        for ret_type in ret_types.iter() {
            if !ret_type.contains(ret_types.first().unwrap().clone()) {
                return Err(type_error("All return types must be the same".to_string()));
            }
        }

        Ok((ret_types[0].clone(), None))
    }
}