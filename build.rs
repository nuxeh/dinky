use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;

fn main() {
    // don't rebuild if source hasn't changed
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=example.html");
    println!("cargo:rerun-if-changed=example.css");

    // write example html and css as consts
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("index.rs");
    let mut f = File::create(&dest_path).unwrap();

    let html = read_to_string("example.html").unwrap();
    let css = read_to_string("example.css").unwrap();

    let mut html = format!(r##"const DEFAULT_INDEX: &str = r#"{}"#;"##, html);
    let mut css = format!(r##"const DEFAULT_CSS: &str = r#"{}"#;"##, css);
    html.push('\n');
    html.push('\n');
    css.push('\n');

    f.write(&html.into_bytes()).unwrap();
    f.write(&css.into_bytes()).unwrap();
}
