#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
fn rocket() -> rocket::Rocket {
  rocket::ignite()
    .mount("/", routes![index])
}

fn main() {
  rocket().launch();
}
