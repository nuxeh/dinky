use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;

fn main() {
    // don't rebuild if source hasn't changed
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=example.html");
    println!("cargo:rerun-if-changed=example.css");

    // get crate version
    let crate_ver = format!("{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"));

    // write example html, submission form html, and css as consts
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("index.rs");
    let mut f = File::create(&dest_path).unwrap();

    let html = read_to_string("example.template.html").unwrap();
    let form = read_to_string("example.form.html").unwrap();
    let css = read_to_string("example.css").unwrap();

    let mut html = format!(r##"const DEFAULT_INDEX: &str = r#"{}"#;"##, html);
    let mut form = format!(r##"const DEFAULT_FORM: &str = r#"{}"#;"##, form);
    let mut css = format!(r##"const DEFAULT_CSS: &str = r#"{}"#;"##, css);

    form.push('\n');
    form.push('\n');
    html.push('\n');
    html.push('\n');
    css.push('\n');
    css.push('\n');

    f.write(&html.into_bytes()).unwrap();
    f.write(&form.into_bytes()).unwrap();
    f.write(&css.into_bytes()).unwrap();

    // write crate version as a const
    let ver = format!(r#"const CRATE_VERSION: &str = "{}";"#, crate_ver);
    f.write(&ver.into_bytes()).unwrap();
}
