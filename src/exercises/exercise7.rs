// src/exercises/exercise7.rs

// Exercise 7: Implement Filter

// Your task is to implement a function `my_filter` that takes a Vector and a predicate function, returning a Vector
// where each item satisfies the predicate function.

// Hints:
// - You can use the iter() method to iterate over the vector items.
// - Try creating a new vector and pushing items into it conditionally.
// - Remember to return the new Vector from the function.

pub fn my_filter<T: Copy>(items: Vec<T>, predicate: fn(T) -> bool) -> Vec<T> {
    unimplemented!() // Replace this with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_filter() {
        assert_eq!(my_filter(vec![1, 2, 3], |x| x > 2), vec![3]);

        assert_eq!(
            my_filter(vec!["apple", "bananas", "lemons"], |&x| x.len() > 5),
            vec!["bananas", "lemons"]
        );
    }

    #[test]
    fn test_my_filter_empty_vec() {
        // Test if my_filter function works correctly on an empty vector.
        let items: Vec<i32> = vec![];
        assert_eq!(my_filter(items, |x| x > 2), vec![]);
    }

    #[test]
    fn test_my_filter_all_items_filtered() {
        // Test if my_filter function works correctly when all items are filtered out.
        let items = vec![1, 2, 3];
        assert_eq!(my_filter(items, |x| x > 5), vec![]);
    }

    #[test]
    fn test_my_filter_strings() {
        // Test if my_filter function works correctly on a vector of strings.
        let items = vec!["apple", "banana", "lemon", "orange"];
        assert_eq!(
            my_filter(items, |&x| x.contains("a")),
            vec!["apple", "banana", "orange"]
        );
    }
}
