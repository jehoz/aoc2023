use std::env;
use std::println;

mod solvers;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("ERROR: Give argument (which day do you want to solve?)");
            return;
        }
        Some(arg) => {
            let day: u32 = arg.parse().expect("Argument must be an integer");
            solvers::run_solver(day);
        }
    }
}
