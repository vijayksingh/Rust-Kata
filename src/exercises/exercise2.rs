/*
Exercise 2: Using for_each to Process a Vector

OVERVIEW:
This exercise repeats the previous one, but now we're emphasizing the use of the for_each function.
In Rust, this is a method provided by the Iterator trait, and it executes a provided closure on each
element of the calling iterator.

For this task, you are required to traverse a vector of strings (names) using the 'for_each' function,
and return them as a single String, each name separated by a newline ("\n").

REFERENCE:
    - Iterators & for_each: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each

NOTE: When you think you're done, you can run the tests using `cargo test --bin exercise2` to verify your solution.
*/

// Boilerplate Code
pub fn get_all_names_foreach(names: Vec<&str>) -> String {
    /*
    HINT: In this function, try using the `for_each` function instead of a direct `for` loop.
    You may want to initially convert the names into an Iterator using the 'iter()' function.
    For the closure passed into 'for_each', you may need to mutate a local variable. To do this,
    you'll have to utilize the 'mut' keyword and consider using a String for concatenation.

    Remember, you are required to add each name to an output string, separating them by "\n".
    */
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_all_names_foreach_test() {
        let names = vec!["Ben", "Jafar", "Matt", "Priya", "Brian"];
        assert_eq!(
            get_all_names_foreach(names),
            "Ben\nJafar\nMatt\nPriya\nBrian\n"
        );
    }
}
