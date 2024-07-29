use rand::Rng;

/// Generate a random number between 0 to a specified limit.
/// e.g. get_rand_value(8) -> 2
pub fn get_rand_value(lim: usize) -> usize {
    rand::thread_rng().gen_range(0..lim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rand_value() {
        match get_rand_value(10) {
            0..=10 => {} // ignore, it means ok,
            _ => panic!("test_get_rand_value() -> get_rand_value()"),
        }
    }
}
