/// Return an iterator over the individual digits in a `usize`,
/// starting with the rightmost digit and progressing left.
pub fn digits(mut n: usize) -> impl Iterator<Item = u8> {
    std::iter::from_fn(move || {
        if n == 0 {
            None
        } else {
            let out = n % 10;
            n = n / 10;
            Some(out as u8)
        }
    })
}