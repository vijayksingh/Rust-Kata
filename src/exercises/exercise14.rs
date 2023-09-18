// src/exercises/exercise14.rs

#[derive(Debug)]
struct Video {
    id: i32,
    title: String,
    boxart: String,
}

#[derive(Debug)]
struct MovieList {
    name: String,
    videos: Vec<Video>,
}

// Implement flat_map_video_info function
fn flat_map_video_info(movie_lists: Vec<MovieList>) -> Vec<Video> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_map_video_info() {
        let movie_lists: Vec<MovieList> = vec![
            MovieList {
                name: String::from("Instant Queue"),
                videos: vec![
                    Video {
                        id: 70111470,
                        title: String::from("Die Hard"),
                        boxarts: vec![
                            Boxart {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/DieHard150.jpg",
                                ),
                            },
                            Boxart {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/DieHard200.jpg",
                                ),
                            },
                        ],
                        url: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                        rating: 4.0,
                        bookmark: vec![],
                    },
                    Video {
                        id: 654356453,
                        title: String::from("Bad Boys"),
                        boxarts: vec![
                            Boxart {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/BadBoys200.jpg",
                                ),
                            },
                            Boxart {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/BadBoys150.jpg",
                                ),
                            },
                        ],
                        url: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
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
                            Boxart {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/TheChamber150.jpg",
                                ),
                            },
                            Boxart {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/TheChamber200.jpg",
                                ),
                            },
                        ],
                        url: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                        rating: 4.0,
                        bookmark: vec![],
                    },
                    Video {
                        id: 675465,
                        title: String::from("Fracture"),
                        boxarts: vec![
                            Boxart {
                                width: 200,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/Fracture200.jpg",
                                ),
                            },
                            Boxart {
                                width: 150,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/Fracture150.jpg",
                                ),
                            },
                            Boxart {
                                width: 300,
                                height: 200,
                                url: String::from(
                                    "http://cdn-0.nflximg.com/images/2891/Fracture300.jpg",
                                ),
                            },
                        ],
                        url: String::from("http://api.netflix.com/catalog/titles/movies/70111470"),
                        rating: 5.0,
                        bookmark: vec![Bookmark {
                            id: 432534,
                            time: 65876586,
                        }],
                    },
                ],
            },
        ];

        let video_info = flat_map_video_info(movie_lists);

        assert_eq!(video_info.len(), 4);

        assert!(video_info.iter().any(|video| {
            video.id == 675465
                && video.title == "Fracture"
                && video.boxart == "http://cdn-0.nflximg.com/images/2891/Fracture150.jpg"
        }));

        assert!(video_info.iter().any(|video| {
            video.id == 65432445
                && video.title == "The Chamber"
                && video.boxart == "http://cdn-0.nflximg.com/images/2891/TheChamber150.jpg"
        }));

        assert!(video_info.iter().any(|video| {
            video.id == 654356453
                && video.title == "Bad Boys"
                && video.boxart == "http://cdn-0.nflximg.com/images/2891/BadBoys150.jpg"
        }));

        assert!(video_info.iter().any(|video| {
            video.id == 70111470
                && video.title == "Die Hard"
                && video.boxart == "http://cdn-0.nflximg.com/images/2891/DieHard150.jpg"
        }));
    }
}
