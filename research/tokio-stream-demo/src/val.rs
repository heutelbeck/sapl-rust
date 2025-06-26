#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Val {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    //String(String),
    None,
}

impl Val {
    pub fn is_number(&self) -> bool {
        matches!(self, Val::Integer(_) | Val::Float(_))
    }
    pub fn is_integer(&self) -> bool {
        matches!(self, Val::Integer(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Val::Float(_))
    }
}
