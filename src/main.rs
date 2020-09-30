#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
use rocket::Request;

use std::str::FromStr;

use reqwest::blocking;

mod response_image;
use response_image::ResponseImage;

impl<'a> Responder<'a> for ResponseImage {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .header(ContentType::from_str("image/png").unwrap())
            .streamed_body(self)
            .ok()
    }
}

#[get("/?<x>&<y>&<size>&<text>&<url>")] 
fn index(url: String, x: u32, y: u32, size: f32, text: String) -> Result<ResponseImage, &'static str> {
    
    let resp = blocking::get(url.as_str());
    if let Err(_) = resp {
        return Err("Malformed URL");
    }

    let resp = resp.unwrap();

    let buf = resp.bytes().unwrap();

    ResponseImage::new(buf, x, y, size, text)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .launch();
}
