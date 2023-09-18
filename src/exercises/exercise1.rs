/*
Exercise 1: Traversing an Array

OVERVIEW:
In this exercise, your task is to traverse a vector (Rust's version of an array) of strings (names),
and return them as a single String, with each name separated by a newline ("\n").

You'll replace all 'unimplemented!' placeholders with your code. As a learning exercise,
Rust's primary documentation should be your first reference. It contains a section dedicated to vectors, which can help guide you on this task.

REFERENCE:
    - Vectors: https://doc.rust-lang.org/std/vec/

NOTE: When you think you're done, you can run the tests using `cargo test --bin exercise1` to verify your solution.
*/

// Boilerplate Code
pub fn get_all_names(names: Vec<&str>) -> String {
    /*
    HINT: In this function, your task is to:
    1. Iterate over all the names in the `names` vector.
    2. As you access each name, add it to an output string.
    3. Remember to add a newline character ("\n") after each name, so they aren't all on one line.
    4. Finally, return the output string.
    */
    unimplemented!()
}

/*
Unit Test

Unit testing is a software testing method by which individual units of source code—sets of one
or more computer program modules together with associated control data, usage procedures, and
operating procedures—are tested to determine whether they are fit for use.

REFERENCE:
  - Testing: https://doc.rust-lang.org/book/ch11-00-testing.html
*/

#[cfg(test)]
mod tests {
    use super::*;

    // Add test annotation to convert normal function to test function
    #[test]
    fn get_all_names_test() {
        let names = vec!["Ben", "Jafar", "Matt", "Priya", "Brian"];
        /*
        HINT: Here, you're expected to replace the empty string "" with the
        expected output -- a single string combining all names (each followed by a newline).
        This will serve as a reference to verify the correctness of your `get_all_names` function.
        */
        assert_eq!(get_all_names(names), "");
    }
}
