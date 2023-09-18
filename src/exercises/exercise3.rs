// src/exercises/exercise3.rs

#[derive(Debug)]
pub struct Video {
    id: u32,
    title: String,
    boxart: String,
    uri: String,
    rating: f32,
    bookmark: Vec<Bookmark>,
}

#[derive(Debug)]
pub struct Bookmark {
    id: u32,
    time: u32,
}

// This struct represents a pair of id and title
#[derive(Debug, PartialEq)]
pub struct IdTitlePair {
    id: u32,
    title: String,
}

// The task is to take a vector of `Video` structs and project it to a
// vector of `IdTitlePair` structs using the `for_each` function.
pub fn project_to_id_title_pairs(videos: Vec<Video>) -> Vec<IdTitlePair> {
    unimplemented!() // Replace this with your implementation.
}

#[cfg(test)]
// ...snip...

// In the unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_to_id_title_pairs() {
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

        let id_title_pairs_expected = vec![
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
            IdTitlePair {
                id: 675465,
                title: String::from("Fracture"),
            },
        ];

        assert_eq!(project_to_id_title_pairs(videos), id_title_pairs_expected);
    }
}
