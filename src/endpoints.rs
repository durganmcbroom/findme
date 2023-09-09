use rocket::{Build, get, put, Rocket, routes};
use rocket::http::{RawStr, Status};
use rocket::serde::json::Json;

use crate::data::{LatLon, Post, PostComment, PostIn, PostStub};
use crate::data::db::PostDatabase;
use crate::DbState;

pub trait RegisterEndpoints {
    fn register_all(self) -> Rocket<Build>;
}

impl RegisterEndpoints for Rocket<Build> {
    fn register_all(self) -> Rocket<Build> {
        self.mount("/posts", routes![
            get_post_stubs,
            get_post_by_id,
            put_post,
            put_comment
        ])
    }
}

#[get("/stubs?<start>&<end>&<lat>&<lon>")]
fn get_post_stubs<'a>(
    start: usize,
    end: usize,
    lat: String,
    lon: String,
    db: &DbState<'a>,
) -> Result<Json<Vec<PostStub>>, Status> {
    let lat = lat.parse::<f64>()
        .map_err(|_| Status::BadRequest)?;

    let lon = lon.parse::<f64>()
        .map_err(|_| Status::BadRequest)?;

    let lat_lon = LatLon {
        lat,
        lon,
    };

    let guard = db.lock().unwrap();

    let vec = (*guard).query_stubs_by_location(
        start, end, lat_lon,
    ).map_err(|_| Status::InternalServerError)?.into_iter().cloned().collect();
    Ok(Json(vec))
}

#[get("/<id>")]
fn get_post_by_id<'a>(
    id: u64,
    db: &DbState<'a>,
) -> Result<Json<Post>, Status> {
    let guard = db.lock().unwrap();

    (*guard).query_post_by_id(id)
        .map(|it| {
            Json(it.clone())
        })
        .ok_or(Status::NotFound)
}


#[put("/", data = "<post>")]
fn put_post<'a>(
    post: Json<PostIn>,
    db: &DbState<'a>,
) -> Result<(), Status> {
    let mut guard = db.lock().unwrap();
    (*guard).insert_post(post.0).map_err(|_| Status::InternalServerError)
}

#[put("/comment?<post>", data = "<comment>")]
fn put_comment<'a>(
    post: u64,
    comment: Json<PostComment>,
    db: &DbState<'a>
) -> Result<(), Status> {

    let mut guard = db.lock().unwrap();
    (*guard).insert_comment(post, comment.0).map_err(|_| Status::NotFound)
}