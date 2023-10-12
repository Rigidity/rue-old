use rue_parser::parse_text;

fn main() {
    let mut all_errors = Vec::new();

    let source = include_str!("../main.rue");

    let (errors, output) = parse_text(source);
    all_errors.extend(errors);

    println!("{:?}", all_errors);
    println!("{:#?}", output);
}
