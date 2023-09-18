// src/exercises/exercise4.rs

// Exercise 4: Implement map()
// We want to implement a 'map' function, which applies a function 'func' to each item in an input vector and outputs a new vector with the results. This manual implementation helps us understand the behavior of the built-in `map`.

// Hints:
// - You should iterate over the input vector. This can be achieved using a loop or the `iter()` method in combination with a `for` loop.
// - Apply 'func' to each item in the vector and collect the results into a new vector.
//   This new vector should be returned as the function result.

// Note: We are not using Rust's built-in `map` function in order to fully understand its internal workings.

pub fn map<T, U, F>(items: Vec<T>, func: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    unimplemented!() // Replace with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        // Test if map function works correctly by mapping `[1,2,3]` to `[2,3,4]`.
        let items = vec![1, 2, 3];
        assert_eq!(map(items, |x| x + 1), vec![2, 3, 4]);
    }

    #[test]
    fn test_map_empty_vec() {
        // Test if map function works correctly on an empty vector.
        let items: Vec<i32> = vec![];
        assert_eq!(map(items, |x| x + 1), vec![]);
    }

    #[test]
    fn test_map_negative_numbers() {
        // Test if map function works correctly on a vector of negative numbers.
        let items = vec![-3, -2, -1];
        assert_eq!(map(items, |x| x.abs()), vec![3, 2, 1]);
    }

    #[test]
    fn test_map_strings() {
        // Test if map function works correctly on a vector of strings.
        let items = vec!["hello", "world", "rust"];
        assert_eq!(map(items, |x| x.len()), vec![5, 5, 4]);
    }
}
