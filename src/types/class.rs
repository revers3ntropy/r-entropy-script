use crate::ast::class_declaration::{
    method_id, operator_method_id,
};
use crate::parse::token::Token;
use crate::types::function::FnType;
use crate::types::Type;
use crate::util::MutRc;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ClassFieldType {
    pub name: String,
    pub type_: MutRc<dyn Type>,
    pub stack_offset: usize,
}
impl ClassFieldType {
    fn str(&self) -> String {
        if self.name == "" {
            self.type_.borrow().str()
        } else {
            format!(
                "{}: {}",
                self.name,
                self.type_.borrow().str()
            )
        }
    }
}

#[derive(Clone)]
pub struct ClassType {
    pub id: usize,
    pub name: String,
    pub fields: HashMap<String, ClassFieldType>,
    pub methods: HashMap<String, MutRc<FnType>>,
    pub is_primitive: bool,
    pub generic_args: HashMap<String, MutRc<dyn Type>>,
    pub generic_params_order: Vec<String>,
}

impl ClassType {
    pub fn field_type(
        &self,
        field: &str,
    ) -> Option<MutRc<dyn Type>> {
        self.fields.get(field).map(|f| f.type_.clone())
    }

    pub fn method_type(
        &self,
        method: &str,
    ) -> Option<MutRc<FnType>> {
        self.methods
            .get(&method_id(
                self.name.clone(),
                method.to_string(),
            ))
            .map(|f| f.clone())
    }

    pub fn field_offset(&self, field: String) -> usize {
        self.fields.get(&field).unwrap().stack_offset
    }

    pub fn concrete_from_abstract(
        &self,
        generic_args: HashMap<String, MutRc<dyn Type>>,
    ) -> ClassType {
        let mut fields = self.fields.clone();

        // Concrete-ify any abstract fields
        let field_names = fields.clone().into_keys();
        for name in field_names {
            let fields_ = fields.clone();
            let field =
                fields_.get(name.as_str()).clone().unwrap();
            let type_ = field.type_.borrow_mut();
            if let Some(generic) = type_.as_generic() {
                fields.insert(
                    name.clone(),
                    ClassFieldType {
                        name: field.name.clone(),
                        type_: generic_args
                            .get(
                                generic
                                    .identifier
                                    .literal
                                    .clone()
                                    .unwrap()
                                    .as_str(),
                            )
                            .unwrap()
                            .clone(),
                        stack_offset: field.stack_offset,
                    },
                );
            }
        }
        ClassType {
            id: self.id,
            name: self.name.clone(),
            fields,
            methods: self.methods.clone(),
            is_primitive: self.is_primitive,
            generic_args,
            generic_params_order: self
                .generic_params_order
                .clone(),
        }
    }
}

impl fmt::Debug for ClassType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{} {{ {} }}",
            self.name,
            self.fields
                .iter()
                .map(|(_, f)| f.str())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Type for ClassType {
    fn is_ptr(&self) -> bool {
        true
    }

    fn str(&self) -> String {
        self.name.clone()
    }

    fn operator_signature(
        &self,
        op: Token,
    ) -> Option<MutRc<FnType>> {
        self.methods
            .get(&operator_method_id(self.name.clone(), op))
            .map(|f| f.clone())
    }

    fn contains(&self, other: MutRc<dyn Type>) -> bool {
        if other.borrow().is_unknown() {
            return true;
        }
        return if let Some(other) =
            other.borrow().as_class()
        {
            if other.id != self.id {
                return false;
            }
            for name in self.generic_args.keys() {
                if !self
                    .generic_args
                    .get(name)
                    .unwrap()
                    .borrow()
                    .contains(
                        other
                            .generic_args
                            .get(name)
                            .unwrap()
                            .clone(),
                    )
                {
                    return false;
                }
            }
            return true;
        } else {
            false
        };
    }

    fn as_class(&self) -> Option<ClassType> {
        Some(self.clone())
    }
}
