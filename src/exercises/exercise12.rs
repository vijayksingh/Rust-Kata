// src/exercises/exercise12.rs

// Exercise 12: Retrieve id, title, and a 150x200 boxart url for every video

// Given a collection of `MovieList` with each containing a collection of `Video`. Each `Video` now contains a collection of `BoxArt`.
// Create a function `filter_map_and_flatten` that selects a structure of `{id, title, boxart}` for every video in the movieLists.
// The `boxart` property in the result will be the url of the boxart object with dimensions of 150x200px.
// Implement this using `map`, `concat_all`, and `filter`.

// Remember: Use preferential access to struct fields, don't use indexers.

struct BoxArt {
    width: u32,
    height: u32,
    url: String,
}

struct Video {
    id: u32,
    title: String,
    boxarts: Vec<BoxArt>,
    uri: String,
    rating: f32,
    bookmark: Vec<Bookmark>,
}

struct Bookmark {
    id: u32,
    time: u32,
}

pub struct MovieList {
    name: String,
    videos: Vec<Video>,
}

pub struct MovieBoxArt {
    id: u32,
    title: String,
    boxart: String,
}

// Implement this function.
pub fn filter_map_and_flatten(movie_lists: Vec<MovieList>) -> Vec<MovieBoxArt> {
    unimplemented!() // Replace with your implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_map_and_flatten() {
        let movie_lists = vec![
            MovieList {
                name: String::from("Instant Queue"),
                videos: vec![
                    Video {
                        id: 70111470,
                        title: String::from("Die Hard"),
                        boxarts: vec![
                            BoxArt {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/DieHard150.jpg",
                                ),
                            },
                            BoxArt {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/DieHard200.jpg",
                                ),
                            },
                        ],
                        uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                        rating: 4.0,
                        bookmark: vec![],
                    },
                    Video {
                        id: 654356453,
                        title: String::from("Bad Boys"),
                        boxarts: vec![
                            BoxArt {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/BadBoys200.jpg",
                                ),
                            },
                            BoxArt {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/BadBoys150.jpg",
                                ),
                            },
                        ],
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
                name: String::from("New Releases"),
                videos: vec![
                    Video {
                        id: 65432445,
                        title: String::from("The Chamber"),
                        boxarts: vec![
                            BoxArt {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/TheChamber150.jpg",
                                ),
                            },
                            BoxArt {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/TheChamber200.jpg",
                                ),
                            },
                        ],
                        uri: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                        rating: 4.0,
                        bookmark: vec![],
                    },
                    Video {
                        id: 675465,
                        title: String::from("Fracture"),
                        boxarts: vec![
                            BoxArt {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/Fracture200.jpg",
                                ),
                            },
                            BoxArt {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/Fracture150.jpg",
                                ),
                            },
                            BoxArt {
                                width: 300,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/Fracture300.jpg",
                                ),
                            },
                        ],
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

        let expected = vec![
            MovieBoxArt {
                id: 675465,
                title: String::from("Fracture"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/Fracture150.jpg"),
            },
            MovieBoxArt {
                id: 65432445,
                title: String::from("The Chamber"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/TheChamber150.jpg"),
            },
            MovieBoxArt {
                id: 654356453,
                title: String::from("Bad Boys"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/BadBoys150.jpg"),
            },
            MovieBoxArt {
                id: 70111470,
                title: String::from("Die Hard"),
                boxart: String::from("http://cdn-0.nflximg.com/images/2891/DieHard150.jpg"),
            },
        ];

        assert_eq!(filter_map_and_flatten(movie_lists), expected);
    }
}
