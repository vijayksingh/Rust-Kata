// src/main.rs
mod exercises {
    pub mod exercise1; // This line includes src/exercises/exercise1.rs as a module
}

fn main() {
    let names = vec!["Ben", "Jafar", "Matt", "Priya", "Brian"];
    exercises::exercise1::get_all_names(names);
}
