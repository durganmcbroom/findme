use std::collections::HashMap;

use crate::data::{FindMeError, LatLon, Post, PostIn, PostStub};

pub trait PostDatabase: Send + Sync {
    fn query_stubs_by_location(
        &self,
        start: usize,
        end: usize,
        lat_lon: LatLon,
    ) -> Result<Vec<&PostStub>, FindMeError>;

    fn query_post_by_id(
        &self,
        id: u64,
    ) -> Option<&Post>;

    fn insert_post(
        &mut self,
        post: PostIn
    ) -> Result<(), FindMeError>;
}

pub struct InMemoryDatabase {
    pub stubs: Vec<PostStub>,
    pub posts: HashMap<u64, Post>,
    pub next_id: u64
}

impl PostDatabase for InMemoryDatabase {
    fn query_stubs_by_location(&self, start: usize, end: usize, lat_lon: LatLon) -> Result<Vec<&PostStub>, FindMeError> {
        let from = lat_lon.to_location();

        Ok(self.stubs.iter()
            .filter(|stub| {
                let dist = from.distance_to(&stub.last_location.to_location())
                    .map_err(|_| FindMeError::FailedToCalculateDistance).ok();

                match dist {
                    None => false,
                    Some(d) => d.meters() < 100f64
                }
            })
            .enumerate()
            .filter(|(i, _)| i >= &start && i < &end)
            .map(|(_, p)| p)
            .collect())
    }

    fn query_post_by_id(&self, id: u64) -> Option<&Post> {
        self.posts.get(&id)
    }

    fn insert_post(&mut self, post: PostIn) -> Result<(), FindMeError> {
        let post = post.to_post(self.next_id);
        self.stubs.push(post.to_stub());
        self.posts.insert(self.next_id, post);
        self.next_id = self.next_id + 1;
        Ok(())
    }
}


// pub fn query_stubs_by_location(
//     start: usize,
//     end: usize,
//     lat_lon: LatLon,
// ) -> Vec<PostStub> {
//     return vec![
//         PostStub {
//             post_id: 0,
//             name: "Test".to_string(),
//             image: Default::default(),
//             description: "This is a test".to_string(),
//             age: 17,
//             last_scene: 213080148,
//             disaster_type: DisasterType::CarCrash,
//         },
//         PostStub {
//             post_id: 1,
//             name: "Another test".to_string(),
//             image: Default::default(),
//             description: "This is also a test".to_string(),
//             age: 27,
//             last_scene: 666,
//             disaster_type: DisasterType::CarCrash,
//         },
//     ];
// }
//
// pub fn query_post_by_id(
//     id: u64
// ) -> Option<Post> {
//     return Some(Post {
//         id,
//         name: "Test".to_string(),
//         image: Default::default(),
//         description: "This is a full test post".to_string(),
//         age: 0,
//         last_scene: 0,
//         disaster_type: DisasterType::CarCrash,
//         comments: vec![],
//         status: PostStatus::NotFound,
//     });
// }