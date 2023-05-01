use std::env;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <problem_number>");
        return;
    }

    solutions::run(args[1].as_str());
}
