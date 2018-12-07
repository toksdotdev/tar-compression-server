extern crate rocket;

use multipart::server::Multipart;
use rocket::data::Data;
use rocket::data::{self, FromData};
use rocket::{
  Outcome::{self, Failure},
  Request,
};
use std::io::{Cursor, Result};

#[derive(Debug)]
pub struct CustomFile {
  pub file_name: String,
  pub content: Vec<u8>,
}

#[derive(Debug)]
pub struct FormMultiPart {
  pub file_id: String,
  pub file: CustomFile,
}

impl FromData for FormMultiPart {
  type Error = io::Result<String>;

  fn from_data(request: &Request, data: Data) -> data::Outcome<Self, io::Result<String>> {
    // All of these errors should be reported
    let ct = request
      .headers()
      .get_one("Content-Type")
      .expect("no content-type");
    let idx = ct.find("boundary=").expect("no boundary");
    let boundary = &ct[(idx + "boundary=".len())..];

    let mut d = Vec::new();
    data.stream_to(&mut d).expect("Unable to read");

    let mut mp = Multipart::with_body(Cursor::new(d), boundary);

    // Custom implementation parts

    let mut alpha = None;
    let mut one = None;
    let mut file = None;

    mp.foreach_entry(|mut entry| match entry.headers.name.as_str() {
      "file_id" => {
        let mut aa = Vec::<u8>::new();
        entry.data.save().force_text().write_to(&mut aa);
        alpha = Some(String::from_utf8(aa).unwrap());
      }
      "file" => {
        let mut aa = Vec::new();
        entry.data.save().write_to(&mut aa);
        file = Some(CustomFile {
          file_name: entry.headers.filename.unwrap(),
          content: aa,
        });
      }

      other => panic!("No known key {}", other),
    }).expect("Unable to iterate");

    let v = FormMultiPart {
      file_id: alpha.expect("alpha not set").to_string(),
      file: file.expect("file not set"),
    };

    Outcome::Success(v)
  }
}
