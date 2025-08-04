use killjoylang::{
    Token,
    Stmt,
    Input,
    InterParse,
    Eval,
};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Cargo run -- <name_file.kj>");
        std::process::exit(1);
    }

    let content_file: &String = &args[1];
    let mut struct_parse_file: Input = Input::new((content_file).to_string());
    let lexer: Result<Vec<Token>, ()> = struct_parse_file.read_input_user();

    println!("{:?}", lexer);

    let mut parse: InterParse = InterParse::new(lexer.expect("REASON"));
    let result_parse: Vec<Stmt> = parse.parse_all();

    let mut eval_struct: Eval = Eval::new(result_parse);
    let _ = eval_struct.intpretation();
}
