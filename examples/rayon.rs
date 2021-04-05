use rayon::ThreadPoolBuilder;

fn main() {
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let n = pool.install(|| fib(20));
    println!("{}", n);

    // asynchronous task
    // pool.spawn(|| println!("new thread"));
}

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`
    return a + b;
}
