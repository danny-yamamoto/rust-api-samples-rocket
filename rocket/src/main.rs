use rocket::{routes, get, launch};
//#[macro_use] extern crate rocket;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello World."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello_world])
}
