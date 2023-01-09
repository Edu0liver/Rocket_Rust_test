#[macro_use] extern crate rocket;

mod delay;
use crate::delay::delay::{delay_route, delay_post};

use std::io;

use rocket::tokio::task::spawn_blocking;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, delay_route, blocking_task])
        .mount("/delay", routes![delay_route, delay_post])
}