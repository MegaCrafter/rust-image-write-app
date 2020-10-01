#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use rocket::response::{self, Response};
use rocket::http::{Status, Header};

mod image_error;
mod response_image;

use image_error::ImageError;
use response_image::ResponseImage;

use reqwest::blocking;

#[get("/?<x>&<y>&<size>&<text>&<url>")] 
fn image(url: String, x: u32, y: u32, size: f32, text: String) -> Result<ResponseImage, ImageError> {

    let resp = blocking::get(url.as_str());
    if let Err(_) = resp {
        return Err(ImageError("Malformed URL"));
    }

    let resp = resp.unwrap();

    let buf = resp.bytes().unwrap();
    
    ResponseImage::new(buf, x, y, size, text)
}

#[get("/github")]
fn github() -> response::Result<'static> {
    Response::build()
        .status(Status::new(301, "Github Redirection"))
        .header(Header::new("Location", "https://github.com/MegaCrafter/rust-image-write-app"))
        .ok()
}

#[get("/")]
fn index() -> String {
    String::from("Still work in progress...")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![image])
    .mount("/", routes![github])
    .mount("/", routes![index])
}

fn main() {
    rocke().launch();
}
