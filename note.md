## パス指定と mount メソッドについて
```
#[get("/hoge")]
fn hoge -> String {
  "HOGE".to_string()
}

fn main () {
  rocket::ignite()
    .mount("/", routes![hoge])     // => /hoge
    .mount("/dir", routes![hoge])  // => /dir/hoge
}
```

`#[get("")]` でパスを指定する
`mount(base, roues)` で base 配下に `get("")` で指定したパスで URI を設定できる

## modularize
module にするときは module の中では src/lib.rs がトップレベルになる
マクロの設定とかアトリビュートの指定を lib.rs でやると下層全部に効く

## rustfmt の設定
`rustfmt.toml` or `.rustfmt.toml` で設定可能
https://github.com/rust-lang/rustfmt

## CircleCI
CircleCI 公式 Docker イメージ
[circleci/rust - Docker Hub](https://hub.docker.com/r/circleci/rust)

clippy が入っていないので別途入れること

rocket crate が `1.33.0-nightly` を minimum で要求しているので
`rustup override set nightly` を叩く必要がある
