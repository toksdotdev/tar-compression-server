#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
extern crate flate2;
extern crate futures;
extern crate multipart;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate tar;
extern crate tokio;

use crate::mpart;
use crate::routes;
use flate2::write::GzEncoder;
use flate2::Compression;
use form_multipart;
use form_multipart::FormMultiPart;
use futures::{future::ok, Future};
use rocket::{
    http::ContentType,
    response::{status::NotFound, Content, NamedFile},
};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tar::Builder;

fn main() {
    let _ = std::fs::create_dir(format!("{}", dotenv!("COMPRESSION_DIRECTORY")));

    rocket::ignite()
        .mount("/api/", routes![index, index_upload])
        .launch();
}

#[get("/file/<file_id>")]
fn index_upload(file_id: String) -> Result<Content<NamedFile>, NotFound<rocket_contrib::Json>> {
    let path = Path::new(&format!("{}/", dotenv!("COMPRESSION_DIRECTORY")))
        .join(format!("{}.tar.gz", &file_id));
    println!("{}", path.to_str().unwrap());

    Ok(Content(
        ContentType::new("application", "x-gzip"),
        NamedFile::open(path.to_str().unwrap()).map_err(move |_| {
            NotFound(rocket_contrib::Json(json!({
                "status": false,
                "message" : format!("Uknown file id: {}", &file_id)
            })))
        })?,
    ))
}

#[post("/upload", data = "<data>")]
fn index(data: FormMultiPart) -> rocket_contrib::Json {
    let file_hash = Arc::new(generate_hash(&data.file.content));

    {
        let hash = Arc::clone(&file_hash); //clone();

        let task = futures::lazy(move || {
            write_data(&data.file.file_name, &data.file.content)
                .and_then(move |stream| compress_to_tar(stream.as_slice(), &hash))
        });

        tokio::run(task);
    }

    rocket_contrib::Json(json!({
        "status": true,
        "message" : "Successfully compressed the file.",
        "data": {
            "file_id" : *file_hash
        }
    }))
}

fn generate_hash(data_stream: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    data_stream.hash(&mut hasher);

    format!("{:x}", hasher.finish())
}

// Configures file headers for compression.
fn write_data(file_name: &str, file_content: &[u8]) -> impl Future<Item = Vec<u8>, Error = ()> {
    // Generates file headers
    let mut header = tar::Header::new_gnu();
    header.set_path(file_name).unwrap();
    header.set_size(file_content.len() as u64);
    header.set_cksum();

    // Create file Builder & append file content to builder.
    let mut builder = Builder::new(Vec::new());
    builder.append(&mut header, file_content).unwrap();
    let data = builder.into_inner().unwrap();

    ok(data)
}

// Create & compress the file into tar.gz.
fn compress_to_tar(data: &[u8], tar_name: &str) -> impl Future<Item = (), Error = ()> {
    // Create a tar file.
    let output_archive = File::create(format!(
        "{base_path}/{tar_name}.tar.gz",
        base_path = dotenv!("COMPRESSION_DIRECTORY"),
        tar_name = tar_name
    )).unwrap();

    // Encode the file content.
    let mut enc = GzEncoder::new(output_archive, Compression::best());
    enc.write_all(data).unwrap();

    // Wrap-up the write.
    enc.finish().unwrap();

    ok(())
}
