use image::ImageOutputFormat::Png;
use image::io::Reader;

use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
use rocket::Request;

use imageproc::drawing::draw_text_mut;

use std::io::{Read, Cursor, Error};
use std::str::FromStr;

use rusttype::{Font, Scale};

use crate::image_error::ImageError;

pub struct ResponseImage {
    bytes: Vec<u8>,
    pointer: usize
}

impl ResponseImage {
    pub fn new(buf: bytes::Bytes, x: u32, y: u32, size: f32, text: String) -> Result<ResponseImage, ImageError> {
        if buf.is_empty() {
            return Err(ImageError("Buffer is empty!"));
        }

        let image = Reader::new(Cursor::new(&buf)).with_guessed_format();
        if let Err(_) = image {
            return Err(ImageError("Unsupported image format"));
        }
        
        let image = image.unwrap().decode();
        if let Err(_) = image {
            return Err(ImageError("Cannot decode image"));
        }

        let mut image = image.unwrap();

        let mut bytes = vec![];

        let font_data: &[u8] = include_bytes!("../assets/Poppins-Medium.ttf");
        let font = Font::try_from_bytes(font_data).unwrap();

        draw_text_mut(&mut image, image::Rgba([0, 210, 225, 255]), x, y, Scale { x: size, y: size },
                                             &font, text.as_str());

        image.write_to(&mut bytes, Png).unwrap();

        let image = ResponseImage {
            bytes,
            pointer: 0
        };

        Ok(image)
    }
}

impl Read for ResponseImage {
    fn read(&mut self, vec: &mut [u8]) -> Result<usize, Error> {
        let mut counter = 0;
        for i in self.pointer .. self.bytes.len() {
            if counter >= vec.len() {
                break;
            }

            vec[counter] = self.bytes[i];
            counter += 1;
        }

        self.pointer += counter;

        Ok(counter)
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