use std::path::PathBuf;
use rocket::{Build, Data, FromForm, get, post, put, Rocket, routes, State};
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::http::{RawStr, Status};
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::data::{LatLon, Post, PostComment, PostIn, PostStub};
use crate::data::db::PostDatabase;
use crate::{DbState, MediaDir};

pub trait RegisterEndpoints {
    fn register_all(self) -> Rocket<Build>;
}

impl RegisterEndpoints for Rocket<Build> {
    fn register_all(self) -> Rocket<Build> {
        self.mount("/posts", routes![
            get_post_stubs,
            get_post_by_id,
            put_post,
            put_comment,
            get_image,
            post_image
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


#[post("/", data = "<post>")]
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
    db: &DbState<'a>,
) -> Result<(), Status> {
    let mut guard = db.lock().unwrap();
    (*guard).insert_comment(post, comment.0).map_err(|_| Status::NotFound)
}

#[derive(FromForm)]
struct ImageForm<'a> {
    file: TempFile<'a>,
    extension: &'a str
}

#[post("/media", data = "<temp>", format="multipart/form-data")]
async fn post_image(
    mut temp: Form<ImageForm<'_>>,
    media_dir: &State<MediaDir>,
) -> Result<String, std::io::Error> {
    let mut relative_path = PathBuf::new();
    // relative_path.push(Uuid::new_v4().to_string());
    relative_path.push(format!("{}.{}", Uuid::new_v4().to_string(), temp.extension));

    let mut filepath = media_dir.0.clone();
    filepath.push(relative_path.clone());

    std::fs::create_dir_all(filepath.parent().unwrap())?;

    temp.file.persist_to(filepath.as_path()).await?;

    return Ok(relative_path.to_str().ok_or(std::io::Error::other("Failed to convert path to string"))?.to_string());
}

#[get("/media/<path..>")]
async fn get_image(
    path: PathBuf,
    media_dir: &State<MediaDir>,
) -> std::io::Result<NamedFile> {
    let mut part_path = media_dir.0.clone();
    part_path.push(path);
    NamedFile::open(
        part_path
    ).await
}