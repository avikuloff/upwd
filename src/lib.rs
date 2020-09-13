use indexmap::IndexSet;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;

/// Generate random password.
///
/// # Examples
/// ```
/// # use indexmap::IndexSet;
/// let pool = "0123456789".chars().collect::<IndexSet<char>>();
/// let password = upwd::generate_password(&pool, 15);
///
/// assert_eq!(password.len(), 15);
/// ```
///
/// # Panics
/// Panics if `pool` is empty.
pub fn generate_password(pool: &IndexSet<char>, length: usize) -> String {
    assert!(!pool.is_empty(), "Pool contains no elements!");

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, pool.len());
            pool[idx]
        })
        .collect()
}

/// Calculates entropy.
/// Returns `f64::NEG_INFINITY` if `pool_size` is 0
///
/// # Examples
/// ```
/// let entropy = upwd::calculate_entropy(12, 64);
///
/// assert_eq!(entropy, 72.0);
/// ```
///
/// # Panics
/// Panics when casting BigUint to f64.
/// This usually means that the entropy will be greater than 1024 bits.
// ToDo Нужно ли возвращать бесконечность?
pub fn calculate_entropy(length: usize, pool_size: usize) -> f64 {
    BigUint::from(pool_size)
        .pow(length as u32)
        .to_f64()
        .expect("Typecast error! Failed to convert BigUint to f64!")
        .log2()
}

/// Calculates the required password length to obtain the given entropy.
///
/// # Examples
/// ```
/// let length = upwd::calculate_length(128.0, 64.0);
///
/// assert_eq!(length.ceil(), 22.0);
/// ```
pub fn calculate_length(entropy: f64, pool_size: f64) -> f64 {
    entropy / pool_size.log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_password_assert_len() {
        let pool = "0123456789".chars().collect::<IndexSet<char>>();
        let password = generate_password(&pool, 15);

        assert_eq!(password.len(), 15);
    }

    #[test]
    #[should_panic(expected = "Pool contains no elements!")]
    fn generate_password_passed_empty_pool() {
        let pool = "".chars().collect::<IndexSet<char>>();

        generate_password(&pool, 15);
    }

    #[test]
    fn calculate_entropy_assert_true() {
        let entropy = calculate_entropy(12, 64);

        assert_eq!(entropy, 72.0);
    }

    #[test]
    fn calculate_entropy_passed_length_is_0() {
        let entropy = calculate_entropy(0, 64);

        assert_eq!(entropy, 0.0)
    }

    #[test]
    fn calculate_entropy_passed_pool_size_is_0() {
        let entropy = calculate_entropy(12, 0);

        assert_eq!(entropy, f64::NEG_INFINITY)
    }

    #[test]
    fn calculate_entropy_passed_pool_size_is_1() {
        let entropy = calculate_entropy(12, 1);

        assert_eq!(entropy, 0.0)
    }

    #[test]
    fn calculate_length_assert_true() {
        let length = calculate_length(128.0, 64.0);

        assert_eq!(length.ceil(), 22.0);
    }

    #[test]
    fn calculate_length_entropy_is_0() {
        let length = calculate_length(0.0, 64.0);

        assert_eq!(length, 0.0);
    }

    #[test]
    fn calculate_length_pool_size_is_0() {
        let length = calculate_length(128.0, 0.0);

        assert_eq!(length, 0.0);
    }
}
