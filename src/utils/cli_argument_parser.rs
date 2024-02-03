use clap::Parser;

#[derive(Parser)]
pub struct CliArguments {
    pub filename: String,
}

pub fn parse_cli_arguments() -> CliArguments {
    return CliArguments::parse();
}