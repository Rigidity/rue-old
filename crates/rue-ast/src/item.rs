mod function_item;
mod use_item;

pub use function_item::*;
pub use use_item::*;

use crate::ast_enum;

ast_enum! { Item,
    Function(FunctionItem),
    Use(UseItem)
}
