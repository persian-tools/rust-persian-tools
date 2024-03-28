use arg_parser::*;
use clap::Parser;

mod arg_parser;
mod wraper;

fn main() {
    let args = Args::parse();

    match args.function {
        Function::AddCommas => {
            handle(&args, wraper::commas::add_commas);
        }
        Function::RemoveCommas => {
            handle(&args, wraper::commas::remove_commas);
        }
    }
}

fn handle(args: &Args, exe: fn(&str) -> String) {
    if let Some(x) = &args.input {
        println!("{}", exe(x));
    } else {
        let mut buffer = String::new();
        loop {
            buffer.clear();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("can't read from std");
            buffer = buffer.trim().to_string();
            if buffer.is_empty() {
                break;
            }
            println!("{}", exe(&buffer));
        }
    }
}
