#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};
use rocket::Request;

use std::str::FromStr;

use reqwest::blocking;

mod response_image;
use response_image::ResponseImage;

struct ImageError(&str);

impl<'a> Responder<'a> for ImageError {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .status(Status::BadRequest)
            .sized_body(self.0)
            .ok()
    }
}

impl<'a> Responder<'a> for ResponseImage {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .header(ContentType::from_str("image/png").unwrap())
            .streamed_body(self)
            .ok()
    }
}

#[get("/?<x>&<y>&<size>&<text>&<url>")] 
fn image(url: String, x: u32, y: u32, size: f32, text: String) -> Result<ResponseImage, &'static str> {

    let resp = blocking::get(url.as_str());
    if let Err(_) = resp {
        return Err("Malformed URL");
    }

    let resp = resp.unwrap();

    let buf = resp.bytes().unwrap();

    ResponseImage::new(buf, x, y, size, text)
}

#[get("/github")]
fn github() -> response::Result<'static> {
    Response::build()
        .status(rocket::http::Status::new(301, "Github Redirection"))
        .header(rocket::http::Header::new("Location", "https://github.com/MegaCrafter/rust-image-write-app"))
        .ok()
}

#[get("/")]
fn index() -> String {
    String::from("Still work in progress...")
}

fn main() {
    rocket::ignite()
    .mount("/", routes![image])
    .mount("/", routes![github])
    .mount("/", routes![index])
    .launch();
}
