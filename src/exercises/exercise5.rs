// src/exercises/exercise5.rs

// Exercise 5: Use the map function to project an array of videos into an array of {id, title} pairs

// You've been given a vector `new_releases` containing structs of type `Video`. Your task is to implement a function `project_videos`
// which takes a Vector of `Video` and returns a Vector of `IdTitlePair`. This effectively projects out only the `id` and `title` fields
// of `Video` and discards the rest.

// Hints:
// - You may use the `iter()` method to iterate over the vector items
// - For each item you'll be creating a new `IdTitlePair` and collect them into a new vector
// - Remember to return the new Vector from the function

pub struct Video {
    id: u32,
    title: String,
    boxart: String,
    uri: String,
    rating: f32,
    bookmark: Vec<u32>,
}

pub struct IdTitlePair {
    id: u32,
    title: String,
}

pub fn project_videos(videos: Vec<Video>) -> Vec<IdTitlePair> {
    unimplemented!() // Replace this with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_videos() {
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
        ];

        let expected = vec![
            IdTitlePair {
                id: 70111470,
                title: String::from("Die Hard"),
            },
            IdTitlePair {
                id: 654356453,
                title: String::from("Bad Boys"),
            },
            IdTitlePair {
                id: 65432445,
                title: String::from("The Chamber"),
            },
        ];

        assert_eq!(project_videos(videos), expected);
    }
}
