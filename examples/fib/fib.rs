#[no_mangle]
pub fn fib(a: u32, b: u32, n: u32) -> u32 {
    if n == 0 { a } else { fib(b, a + b, n - 1) }
}
