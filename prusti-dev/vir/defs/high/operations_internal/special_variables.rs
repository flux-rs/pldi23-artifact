use super::super::ast::{
    expression::{Expression, Local},
    field::FieldDecl,
    ty::{self, Type},
    type_decl::Enum,
    variable::VariableDecl,
};

impl Expression {
    pub fn discriminant() -> Self {
        let variable = VariableDecl::new("discriminant$", Type::MInt);
        Expression::local_no_pos(variable)
    }
    pub fn is_discriminant(&self) -> bool {
        if let Expression::Local(Local { variable, .. }) = self {
            variable.name == "discriminant$"
        } else {
            false
        }
    }
}

impl FieldDecl {
    pub fn discriminant(ty: Type) -> Self {
        FieldDecl::new("discriminant", ty)
    }
    pub fn is_discriminant(&self) -> bool {
        self.name == "discriminant"
    }
}

impl Enum {
    pub fn discriminant_field(&self) -> FieldDecl {
        FieldDecl::discriminant(self.discriminant_type.clone())
    }
}
