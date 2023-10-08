mod binary_expr;
mod call_expr;
mod if_expr;
mod literal_expr;
mod prefix_expr;

pub use binary_expr::*;
pub use call_expr::*;
pub use if_expr::*;
pub use literal_expr::*;
pub use prefix_expr::*;

use crate::ast_enum;

ast_enum! { Expr,
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Prefix(PrefixExpr),
    Call(CallExpr),
    If(IfExpr),
}
