// src/exercises/exercise9.rs

// Exercise 9: Flatten the MovieLists Vector into a Vector of Video Ids

// You've been given a vector `movie_lists` containing structs of type `MovieList`. Each `MovieList` has a name and a vector of `Video`.
// Your task is to implement a function `flatten_movie_lists` that takes a Vector of `MovieList` and returns a Vector of `u32`,
// representing the id of each video.

// Hints:
// - Use nested `iter` calls to iterate over the vector within the vector.
// - `Video` ids can be collected into a new vector.

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

struct MovieList {
    name: String,
    videos: Vec<Video>,
}

pub fn flatten_movie_lists(movie_lists: Vec<MovieList>) -> Vec<u32> {
    unimplemented!() // Replace this with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_movie_lists() {
        let movie_lists = vec![
            MovieList {
                name: String::from("New Releases"),
                videos: vec![
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
                ],
            },
            MovieList {
                name: String::from("Dramas"),
                videos: vec![
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
                ],
            },
        ];

        let expected = vec![70111470, 654356453, 65432445, 675465];

        assert_eq!(flatten_movie_lists(movie_lists), expected);
    }
}
