// src/exercises/exercise13.rs

// Exercise 13: Implement flat_map()

// Implement a flat_map function. This function should take in two parameters:
// a Vec<T> and a function that maps from T -> Vec<String>.
// Your task is to apply this function to every item in the given vector and flatten the results into a single Vec<String>.

pub fn flat_map<T, F>(_items: Vec<T>, _func: F) -> Vec<String>
where
    F: Fn(T) -> Vec<String>,
{
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_map() {
        let spanish_french_english_words = vec![
            vec![
                String::from("cero"),
                String::from("rien"),
                String::from("zero"),
            ],
            vec![String::from("uno"), String::from("un"), String::from("one")],
            vec![
                String::from("dos"),
                String::from("deux"),
                String::from("two"),
            ],
        ];

        let all_words = flat_map(vec![0, 1, 2], |index| {
            spanish_french_english_words[index].clone()
        });
        assert_eq!(
            all_words,
            vec![
                String::from("cero"),
                String::from("rien"),
                String::from("zero"),
                String::from("uno"),
                String::from("un"),
                String::from("one"),
                String::from("dos"),
                String::from("deux"),
                String::from("two")
            ]
        );
    }
}
