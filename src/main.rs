use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rocket::{launch, Rocket, State};
use rocket::fairing::Fairing;
use rocket_cors::{Cors, CorsOptions};

use crate::data::db::{InMemoryDatabase, PostDatabase};
use crate::endpoints::RegisterEndpoints;

mod data;
mod endpoints;

pub type DbState<'a> = State<Arc<Mutex<Box<dyn PostDatabase>>>>;

#[launch]
fn launch() -> _ {
    let cors_options = CorsOptions::default();

    let cors : Cors = cors_options.to_cors().unwrap();

    Rocket::build()
        .attach(cors)
        .register_all()
        .manage(Arc::new(Mutex::new(Box::new(InMemoryDatabase {
            stubs: Vec::new(),
            posts: HashMap::new(),
            next_id: 0
        }) as (Box<dyn PostDatabase>))))
}
