use indexmap::IndexSet;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;

pub mod config;

/// Collection of unique chars
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pool(IndexSet<char>);

impl Pool {
    /// Create new empty pool
    pub fn new() -> Self {
        Pool(IndexSet::new())
    }

    /// Create new pool from [`std::string::String`]
    pub fn from_string(s: String) -> Self {
        Pool(s.chars().collect::<IndexSet<char>>())
    }

    /// Return number of chars in the pool
    ///
    /// # Examples
    /// ```
    /// # use indexmap::IndexSet;
    /// let pool = "0123456789".chars().collect::<IndexSet<char>>();
    ///
    /// assert_eq!(pool.len(), 10)
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn extend_from_string(&mut self, s: String) -> &mut Self {
        self.0.extend(s.chars().collect::<IndexSet<char>>());

        self
    }

    /// Returns true if pool contains no elements
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get char by index
    pub fn get(&self, index: usize) -> Option<&char> {
        self.0.get_index(index)
    }

    pub fn contains(&self, element: char) -> bool {
        self.0.contains(&element)
    }

    pub fn contains_all(&self, elements: String) -> bool {
        self.0.is_superset(&elements.chars().collect::<IndexSet<char>>())
    }
}

/// Generate random password.
///
/// # Examples
/// ```
/// # use indexmap::IndexSet;
/// use upwd::Pool;
/// let pool = "0123456789".to_owned();
/// let password = upwd::generate_password(&Pool::from_string(pool), 15);
///
/// assert_eq!(password.len(), 15);
/// ```
///
/// # Panics
/// Panics if `pool` is empty.
pub fn generate_password(pool: &Pool, length: usize) -> String {
    assert!(!pool.is_empty(), "Pool contains no elements!");

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, pool.len());
            *pool.get(idx).unwrap()
        })
        .collect()
}

/// Calculates entropy. Maximum value `f64::MAX`
///
/// # Examples
/// ```
/// let entropy = upwd::calculate_entropy(12, 64);
///
/// assert_eq!(entropy, 72.0);
/// ```
///
/// # Panics
/// Panics if `pool_size` is zero
pub fn calculate_entropy(length: usize, pool_size: usize) -> f64 {
    assert!(pool_size > 0, "Pool size must be greater than zero!");

    BigUint::from(pool_size)
        .pow(length as u32)
        .to_f64()
        .unwrap_or(f64::MAX)
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
        let password = generate_password(&Pool(pool), 15);

        assert_eq!(password.len(), 15);
    }

    #[test]
    #[should_panic(expected = "Pool contains no elements!")]
    fn generate_password_passed_empty_pool() {
        let pool = "".chars().collect::<IndexSet<char>>();

        generate_password(&Pool(pool), 15);
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
    #[should_panic(expected = "Pool size must be greater than zero!")]
    fn calculate_entropy_passed_pool_size_is_0() {
        calculate_entropy(12, 0);
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
