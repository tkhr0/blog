use super::controller;

pub fn rocket() -> rocket::Rocket {
  rocket::ignite()
    .mount("/", routes![controller::top::top])
    .mount(
      "/articles",
      routes![
        controller::articles::articles,
        controller::articles::article
      ],
    )
}
