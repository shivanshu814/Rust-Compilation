#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate tempfile;

use tempfile::NamedTempFile;
use rocket_contrib::json::Json;
use rocket::response::NamedFile;
use std::path::Path;
use std::process::Command;
use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use rocket::Data;
use std::io::Read;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PythonCodeForm {
    python_code: String,
}

#[post("/execute-python", data = "<form>")]
fn execute_python(form: Data, content_type: &ContentType) -> String {
    let mut python_code = String::new();
    if content_type.is_json() {
        if let Ok(_) = form.open().read_to_string(&mut python_code) {
            // Create a temporary file and write the Python code to it
            let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
            let temp_file_path = temp_file.path();
            std::fs::write(&temp_file_path, python_code).expect("Failed to write to temporary file");

            // Execute Python interpreter with the temporary file as input
            let output = Command::new("python")
                .arg(&temp_file_path)
                .output()
                .expect("Failed to execute Python code");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            return format!("Python output:\n{}\nPython error:\n{}", stdout, stderr);
        }
    }

    String::from("Failed to read JSON data or execute Python code.")
}

#[get("/")]
fn home() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/compiler")]
fn compiler() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/compiler.html")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![home, compiler, execute_python])
        .attach(AdHoc::on_attach("Python Code Executor Configuration", |rocket| {
            Ok(rocket)
        }))
        .launch();
}
