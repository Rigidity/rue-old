#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Error,
    Named { name: String },
}
