#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use rocket::response::{self};

mod image_error;
mod response_image;

use image_error::ImageError;
use response_image::ResponseImage;

use reqwest::blocking;

#[get("/?<x>&<y>&<size>&<text>&<url>")] 
fn image(url: String, x: u32, y: u32, size: f32, text: String) -> Result<ResponseImage, ImageError> {

    // We need a blocking request because the image()
    // function is called by Rocket and is already async by itself.
    let resp = blocking::get(url.as_str());
    if let Err(_) = resp {
        return Err(ImageError("Malformed URL"));
    }

    let resp = resp.unwrap();

    // Read the raw bytes from the response to get the image.
    let buf = resp.bytes().unwrap();
    
    // Let ResponseImage do all the other image logic.
    ResponseImage::new(buf, x, y, size, text)
}

#[get("/github")]
fn github() -> response::Redirect {
    // Redirecting to the github page
    response::Redirect::to("https://github.com/MegaCrafter/rust-image-write-app")
}

#[get("/")]
fn index() -> String {
    // TODO: Add changelog things (static pages?)
    String::from("Still work in progress...")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![image])
    .mount("/", routes![github])
    .mount("/", routes![index])
}

#[cfg(test)]
mod test;

fn main() {
    rocket().launch();
}
