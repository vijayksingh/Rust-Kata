// src/exercises/exercise6.rs

// Exercise 6: Collect only those videos with a rating of 5.0

// You've been given a vector `new_releases` containing structs of type `Video`. Your task is to implement a function `filter_videos`
// which takes a Vector of `Video` and returns a Vector of `Video` where each video has a rating of 5.0.

// Hints:
// - You can use the `iter()` method to iterate over the vector items
// - Try creating a new vector and pushing items into it conditionally (if the rating is 5.0)
// - Remember to return the new Vector from the function

#[derive(Debug, PartialEq)]
pub struct Video {
    id: u32,
    title: String,
    boxart: String,
    uri: String,
    rating: f32,
    bookmark: Vec<u32>,
}

pub fn filter_videos(videos: Vec<Video>) -> Vec<Video> {
    unimplemented!() // Replace this with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_videos() {
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
                bookmark: vec![432534],
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
                bookmark: vec![432534],
            },
        ];

        let expected = vec![
            Video {
                id: 654356453,
                title: String::from("Bad Boys"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/BadBoys.jpg"),
                uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                rating: 5.0,
                bookmark: vec![432534],
            },
            Video {
                id: 675465,
                title: String::from("Fracture"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/Fracture.jpg"),
                uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                rating: 5.0,
                bookmark: vec![432534],
            },
        ];

        assert_eq!(filter_videos(videos), expected);
    }
}
