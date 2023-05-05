use std::fmt;

pub fn dfs(numbers: Vec<i32>, target: i32) -> Option<Vec<Expression>> {
    let hist: Vec<Expression> = vec![];
    let mut search = vec![(numbers, hist)];

    while search.len() > 0 {
        if let Some((nums, history)) = search.pop() {
            for (i1, n1) in nums.iter().enumerate() {
                for (i2, n2) in nums.iter().enumerate().filter(|(i, _)| *i != i1) {
                    for op in ALL_OPS.iter() {
                        if let Some(result) = apply_op(*op, *n1, *n2) {
                            let mut new_history = history.clone();
                            new_history.push(Expression(*n1, *op, *n2));
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Ops {
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Expression(i32, Ops, i32);

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Expression(n1, op, n2) = *self;
        write!(f, "{} {} {}", n1, op, n2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // so that you can run this with `cargo test --release -- my_bench --nocapture`
    fn bench() {
        let numbers = vec![3, 5, 7, 13, 20, 25];
        let target = 252013;
        let num_iters = 10;
        let t = std::time::Instant::now();
        for _ in 0..num_iters {
            // adjust the loop time manually, so that the total time is about 1 second
            let res = dfs(numbers.clone(), target);
            // By checking result you make sure that compiler doesn't optimize it away,
            // and also ensure that your optimizations don't break semantics
            assert_eq!(res, None);
        }
        // Print time with two digit after ., to avoid excessive precision
        eprintln!(
            "Avg times over {} iterations: {:0.2?}",
            num_iters,
            t.elapsed() / num_iters
        )
    }

    #[test]
    fn test_dfs_succeeds() {
        let numbers = vec![3, 5, 7, 13, 20, 25];
        let target = 252;
        assert!(matches!(dfs(numbers, target), Some(_)));
    }

    #[test]
    fn test_dfs_fails_impossible() {
        let numbers = vec![3, 5, 7, 13, 20, 25];
        let target = 252013;
        assert!(matches!(dfs(numbers, target), None));
    }

    #[test]
    fn test_dfs_succeeds_with_zero_target() {
        let numbers = vec![3, 5, 7, 13, 20, 25];
        let target = 0;
        assert!(matches!(dfs(numbers, target), Some(_)));
    }

    #[test]
    fn test_dfs_succeeds_with_negative_target() {
        let numbers = vec![3, 5, 7, 13, 20, 25];
        let target = -2;
        assert!(matches!(dfs(numbers, target), Some(_)));
    }

    #[test]
    fn test_dfs_succeeds_with_zero_num() {
        let numbers = vec![3, 0, 7, 13, 20, 25];
        let target = 430;
        assert!(matches!(dfs(numbers, target), Some(_)));
    }

    #[test]
    fn test_dfs_succeeds_with_duplicate_zeroes() {
        let numbers = vec![0, 0, 7, 13, 20, 25];
        let target = 40;
        assert!(matches!(dfs(numbers, target), Some(_)));
    }
}
