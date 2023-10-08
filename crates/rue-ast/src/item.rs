mod fn_item;

pub use fn_item::*;

use crate::ast_enum;

ast_enum! { Item,
    Fn(FnItem),
}
