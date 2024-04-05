use arg_parser::*;
use clap::Parser;

mod arg_parser;
mod constants;
mod wrapper;

fn main() {
    let args = Args::parse();

    match args.function {
        Function::AddCommas => {
            handle(&args, wrapper::add_commas);
        }
        Function::RemoveCommas => {
            handle(&args, wrapper::remove_commas);
        }
    }
}

fn handle(args: &Args, exe: fn(&str, &Args) -> String) {
    if let Some(x) = &args.input {
        println!("{}", exe(x, &args));
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
            println!("{}", exe(&buffer, &args));
        }
    }
}
