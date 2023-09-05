use rue_parser::{FnDef, Item, Program};

pub fn enter_program(program: Program) {
    println!("Entering program");

    for item in program.items() {
        enter_item(item);
    }
}

fn enter_item(item: Item) {
    println!("Entering item");

    match item {
        Item::FnDef(fn_def) => enter_fn_def(fn_def),
    }
}

fn enter_fn_def(fn_def: FnDef) {
    println!("Entering fn def");
    println!("Name = {:?}", fn_def.name());
}
