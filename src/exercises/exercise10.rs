// src/exercises/exercise10.rs

// Exercise 10: Implement concat_all

// Implement a function `concat_all` that takes a nested Vector and returns a flattened Vector.
// `concat_all` function iterates over each sub-vector in the nested vector and collects the results into a new, flat Vector.

// Hints:
// - You'll probably need to use nested iter() calls to achieve this.
// - You can push items to the `results` Vector using the push method.

// Note: This function should panic when called with a one-dimensional array, so don't worry about that case.

pub fn concat_all<T: Copy>(nested: Vec<Vec<T>>) -> Vec<T> {
    unimplemented!() // Replace this with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_all() {
        assert_eq!(
            concat_all(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[should_panic]
    #[test]
    fn test_concat_all_panic() {
        concat_all(vec![vec![1, 2, 3]]);
    }
}
