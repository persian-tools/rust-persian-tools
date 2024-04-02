use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// if you don't pass --input param it will take std-in as input
    #[arg(short, long)]
    pub input: Option<String>,

    /// name of function see function list by running --help
    #[command(subcommand)]
    pub function: Function,
}

#[derive(Debug, Subcommand)]
pub enum Function {
    /// adds commas to number, example: 3100 -> 3,100
    AddCommas,
    /// remove commas from number, example: 3,100 -> 3100
    RemoveCommas,
}
