use clap::Parser;
use nytimes_digits_solver::search::dfs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target number, e.g. 252
    #[arg(short, long, required = true)]
    target: i32,

    /// Numbers available to be combined to reach the target, e.g. 3 5 7 13 20 25
    #[arg(short, long, num_args = 6, required = true)]
    numbers: Vec<i32>,
}

fn main() {
    let args = Args::parse();

    match dfs(args.numbers, args.target) {
        Some(hist) => {
            println!("Solution found!");
            for line in hist {
                println!("{}", line);
            }
        }
        None => println!("No solution found"),
    }
}
