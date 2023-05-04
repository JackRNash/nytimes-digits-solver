use clap::Parser;
use std::fmt;

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

fn dfs(numbers: Vec<i32>, target: i32) -> Option<Vec<String>> {
    let hist: Vec<String> = vec![];
    let mut search = vec![(numbers, hist)];

    while search.len() > 0 {
        if let Some((nums, history)) = search.pop() {
            for (i1, n1) in nums.iter().enumerate() {
                for (i2, n2) in nums.iter().enumerate().filter(|(i, _)| *i != i1) {
                    for op in ALL_OPS.iter() {
                        if let Some(result) = apply_op(*op, *n1, *n2) {
                            let mut new_history = history.clone();
                            new_history.push(format!("{} {} {}", n1, op, n2));
                            if result == target {
                                return Some(new_history);
                            } else {
                                let mut new_nums = nums
                                    .clone()
                                    .iter()
                                    .enumerate()
                                    .filter(|(i, _)| *i != i1 && *i != i2)
                                    .map(|(_, n)| *n)
                                    .collect::<Vec<i32>>();
                                new_nums.push(result);
                                search.push((new_nums, new_history));
                            }
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn apply_op(op: Ops, n1: i32, n2: i32) -> Option<i32> {
    match op {
        Ops::Plus => Some(n1 + n2),
        Ops::Minus => Some(n1 - n2),
        Ops::Multiply => Some(n1 * n2),
        Ops::Divide => {
            if n2 != 0 && n1 % n2 == 0 {
                Some(n1 / n2)
            } else {
                None
            }
        }
    }
}

const ALL_OPS: [Ops; 4] = [Ops::Plus, Ops::Minus, Ops::Multiply, Ops::Divide];

#[derive(Clone, Copy)]
enum Ops {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl fmt::Display for Ops {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ops::Plus => write!(f, "+"),
            Ops::Minus => write!(f, "-"),
            Ops::Multiply => write!(f, "*"),
            Ops::Divide => write!(f, "/"),
        }
    }
}
