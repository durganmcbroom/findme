use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rocket::{launch, Rocket, State};
use rocket::yansi::Color::Default;
use crate::data::db::{InMemoryDatabase, PostDatabase};
use crate::data::{DisasterType, LatLon, PostStub};
use crate::endpoints::RegisterEndpoints;

mod data;
mod endpoints;

pub type DbState<'a> = State<Arc<Mutex<Box<dyn PostDatabase>>>>;

#[launch]
fn launch() -> _ {
    Rocket::build()
        .register_all()
        .manage(Arc::new(Mutex::new(Box::new(InMemoryDatabase {
            stubs: Vec::new(),
            posts: HashMap::new(),
            next_id: 0
        }) as (Box<dyn PostDatabase>))))
}
