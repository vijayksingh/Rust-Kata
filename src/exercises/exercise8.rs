// src/exercises/exercise8.rs

// Exercise 8: Chain Filter and Map

// You've been given a vector `new_releases` containing structs of type `Video`. Your task is to implement a function `filter_and_project`
// which takes a Vector of `Video` and returns a Vector of u32, representing the id of each video with a rating of 5.0.

// Hints:
// - Use the `iter()` method to iterate over the vector items.
// - Use `filter` and `map` combinators to both filer and transform video items.
// - Remember to return the new Vector from the function.

pub struct Video {
    id: u32,
    title: String,
    boxart: String,
    uri: String,
    rating: f32,
    bookmark: Vec<Bookmark>,
}

struct Bookmark {
    id: u32,
    time: u32,
}

pub fn filter_and_project(videos: Vec<Video>) -> Vec<u32> {
    unimplemented!() // Replace this with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_and_project() {
        let videos = vec![
            Video {
                id: 70111470,
                title: String::from("Die Hard"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/DieHard.jpg"),
                uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                rating: 4.0,
                bookmark: vec![],
            },
            Video {
                id: 654356453,
                title: String::from("Bad Boys"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/BadBoys.jpg"),
                uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                rating: 5.0,
                bookmark: vec![Bookmark {
                    id: 432534,
                    time: 65876586,
                }],
            },
            Video {
                id: 65432445,
                title: String::from("The Chamber"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/TheChamber.jpg"),
                uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                rating: 4.0,
                bookmark: vec![],
            },
            Video {
                id: 675465,
                title: String::from("Fracture"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/Fracture.jpg"),
                uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                rating: 5.0,
                bookmark: vec![Bookmark {
                    id: 432534,
                    time: 65876586,
                }],
            },
        ];

        let expected = vec![654356453, 675465];

        assert_eq!(filter_and_project(videos), expected);
    }
}
