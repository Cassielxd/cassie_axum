use cached::proc_macro::cached;

#[cached]
fn fib(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    print!("{}", fib(64))
}
