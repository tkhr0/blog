#[get("/")]
pub fn articles() -> String {
  "article index".to_string()
}

#[get("/<id>")]
pub fn article(id: usize) -> String {
  format!("{}", id)
}
