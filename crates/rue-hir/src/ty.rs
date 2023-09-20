#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    String,
    Fn {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
}
