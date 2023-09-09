use std::path::PathBuf;
use geoutils::Location;
use rocket::serde::Serialize;
use serde::Deserialize;

pub mod db;

#[derive(Serialize, Deserialize, Clone)]
pub enum DisasterType {
    CarCrash,

}

#[derive(Serialize, Deserialize, Clone)]
pub enum PostStatus {
    NotFound,
    Injured(u8),
    Found
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PostCommentType {
    Extra,
    Informational,
    Concrete,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostComment {
    author: String,
    value: String,
    status: PostCommentType
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: u64,
    pub name: String,
    pub image: PathBuf,
    pub description: String,
    pub age: u16,
    pub last_seen: u64,
    pub disaster_type: DisasterType,
    pub last_location: LatLon,
    pub comments: Vec<PostComment>,
    pub status: PostStatus
}

impl Post {
    fn to_stub(&self) -> PostStub {


        return PostStub {
            post_id: self.id.clone(),
            name: self.name.clone(),
            image: self.image.clone(),
            description: self.description.clone(),
            age: self.age.clone(),
            last_seen: self.last_seen.clone(),
            last_location: self.last_location.clone(),
            disaster_type: self.disaster_type.clone(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct PostStub {
    pub post_id: u64,
    pub name: String,
    pub image: PathBuf,
    pub description: String,
    pub age: u16,
    pub last_seen: u64,
    pub last_location: LatLon,
    pub disaster_type: DisasterType,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64
}

impl LatLon {
    pub fn to_location(&self) -> Location {
        Location::new(self.lat, self.lon)
    }
}

pub enum FindMeError {
    FailedToCalculateDistance
}

#[derive(Deserialize)]
pub struct PostIn {
    name: String,
    image: PathBuf,
    description: String,
    age: u16,
    last_seen: u64,
    disaster_type: DisasterType,
    last_location: LatLon,
}

impl PostIn {
    fn to_post(self, id: u64) -> Post {
        return Post {
            id,
            name: self.name.clone(),
            image: self.image.clone(),
            description: self.description.clone(),
            age: self.age.clone(),
            last_seen: self.last_seen.clone(),
            last_location: self.last_location.clone(),
            comments: vec![],
            disaster_type: self.disaster_type.clone(),
            status: PostStatus::NotFound,
        }
    }
}
