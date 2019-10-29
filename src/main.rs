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

#[cfg(test)]
mod test {
  use super::rocket;
  use rocket::http::Status;
  use rocket::local::Client;

  #[test]
  fn top() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
  }

  #[test]
  fn articles() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/articles").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("article index".into()));
  }

  #[test]
  fn article_1() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/articles/1").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("1".into()));
  }

  #[test]
  fn article_2() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/articles/2").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("2".into()));
  }
}
