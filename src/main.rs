mod utils;
mod lexing;

use std::fs::File;
use utils::cli_argument_parser::parse_cli_arguments;
use lexing::{token::Token, token_type::TokenType, bracket::Bracket};


fn main() {
    let token: Token = Token { token_type: TokenType::Bracket(Bracket::LeftCurly) } ;
    let args = parse_cli_arguments();
    let file = File::open(args.filename);
    println!("{}", token);
    match file {
        Ok(f) => println!("{}", f.metadata().unwrap().len()),
        Err(e) => println!("{}", e)
    }
}
