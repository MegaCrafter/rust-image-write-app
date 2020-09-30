use rocket::response::{self, Response, Responder};
use rocket::http::Status;
use rocket::Request;

use std::io::Cursor;

#[derive(Debug)]
pub struct ImageError(pub &'static str);

impl<'a> Responder<'a> for ImageError {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .status(Status::BadRequest)
            .sized_body(Cursor::new(self.0))
            .ok()
    }
}