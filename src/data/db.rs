use std::collections::HashMap;

use crate::data::{FindMeError, LatLon, Post, PostComment, PostIn, PostStatus, PostStub};

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
        post: PostIn,
    ) -> Result<(), FindMeError>;

    fn insert_comment(
        &mut self,
        post_id: u64,
        comment: PostComment,
    ) -> Result<(), FindMeError>;

    fn update_status(
        &mut self,
        post_id: u64,
        status: PostStatus,
    ) -> Result<(), FindMeError>;
}

pub struct InMemoryDatabase {
    pub stubs: Vec<PostStub>,
    pub posts: HashMap<u64, Post>,
    pub next_id: u64,
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

    fn insert_comment(
        &mut self,
        post_id: u64,
        comment: PostComment,
    ) -> Result<(), FindMeError> {
        self.posts
            .get_mut(&post_id)
            .ok_or(FindMeError::UnknownPost(post_id))?
            .comments
            .push(comment);

        Ok(())
    }

    fn update_status(
        &mut self,
        post_id: u64,
        status: PostStatus,
    ) -> Result<(), FindMeError> {
        self.posts
            .get_mut(&post_id)
            .ok_or(FindMeError::UnknownPost(post_id))?
            .status = status;

        Ok(())
    }
}