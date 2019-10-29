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

## rustfmt の設定
`rustfmt.toml` or `.rustfmt.toml` で設定可能
https://github.com/rust-lang/rustfmt
