use memoize::memoize;

#[derive(Eq, PartialEq, Hash, Clone)]
struct FibProblem {
    input: u32,
}

#[memoize]
fn fib(input: FibProblem) -> u128 {
    match input {
        FibProblem { input: n } if n < 2 => u128::from(n),
        FibProblem { input: n } => {
            fib(FibProblem { input: n - 1 }) + fib(FibProblem { input: n - 2 })
        }
    }
}

fn main() {
    for n in 0..100 {
        println!("fib({n}) = {}", fib(FibProblem { input: n }));
    }
}
