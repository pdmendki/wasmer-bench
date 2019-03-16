#[no_mangle]
pub extern fn fib(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fib(n-1) + fib(n-2),
    }
}