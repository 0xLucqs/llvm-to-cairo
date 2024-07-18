#[no_mangle]
pub fn increment(left: u128) -> u128 {
    left + u128::MAX / 2
}
