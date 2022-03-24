extern crate clap;
extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate serde_json;

use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use image::{Rgb, RgbImage};
use rusttype::{Font, Scale};
use serde_json::Value;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use clap::{Command, Arg};

pub fn main() {
  let matches = Command::new("faviconr")
    .author("Michael Borejdo")
    .about("generates favicons")
    .arg_required_else_help(false)
    .arg( Arg::new("offset")
            .help("horizontal offset")
            .long("offset")
            .short('o')
            .takes_value(true)
            .required(false))
    .arg( Arg::new("scale")
            .help("scale")
            .long("scale")
            .short('s')
            .takes_value(true)
            .required(false))
    .arg( Arg::new("font")
            .help("font")
            .long("font")
            .short('f')
            .takes_value(true)
            .required(false));


  let matches = matches.get_matches();
  if let Some(c) = matches.value_of("font") {
      println!("Value for -font: {}", c);
  }

  let conf_file = "src/conf.json";
  let icon_text = parse_json(conf_file, "text").to_string().replace("\"", "");
  let sizes     = parse_json(conf_file, "sizes");

  for i in 0..100 {
    if sizes[i].is_null() { break; }
    
    let img_size = sizes[i]["pixels"].to_string().parse::<u32>().unwrap();
    let img_format = sizes[i]["format"].to_string().replace("\"", "");

    let meta = "";

    create_favicon(
      &icon_text,
      &format!("./output/favicon{}.{}", &img_size, &img_format),
      img_size, 
      meta
    );
  }

}

pub fn create_favicon(txt: &str, filepath: &str, dimensions: u32, meta: &str) {
  let black = colorsys::Rgb::from_hex_str("#000000").unwrap_or(colorsys::Rgb::from((0.0, 0.0, 0.0)));
  let white = colorsys::Rgb::from_hex_str("#ffffff").unwrap_or(colorsys::Rgb::from((255.0, 255.0, 255.0)));
  let bg = Rgb([black.red() as u8, black.green() as u8, black.blue() as u8]);
  let fg = Rgb([white.red() as u8, white.green() as u8, white.blue() as u8]);

  let font_offset = 1u32;
  let path = Path::new(filepath);
  let mut image = RgbImage::new(dimensions, dimensions);

  draw_filled_rect_mut(
    &mut image,
    Rect::at(font_offset as i32, font_offset as i32).of_size(dimensions - font_offset, dimensions - font_offset),
    bg
  );

  let font = Vec::from(include_bytes!("./assets/DejaVuSans.ttf") as &[u8]);
  let font = Font::try_from_vec(font).unwrap();

  // USE CLAP
  let horizontal_compression: f32 = match env::args().nth(2) {
    Some(h_compression) => {
      h_compression.parse::<f32>().unwrap_or(80.0) / 100 as f32
    },
    _ =>
      0.8
  };
  let vertical_compression = 0.8;
 

  let scale = Scale {
    x: (dimensions as f32) * horizontal_compression,
    y: (dimensions as f32) * vertical_compression
  };

  // USE CLAP
  let horizontal_offset: u32 = match env::args().nth(1) {
    Some(h_offset) => {
      let percent_offset = h_offset.parse::<f32>().unwrap_or(0.00) / 100 as f32;
      (percent_offset * dimensions as f32) as u32
    },
    _ =>
      0
  };
  let vertical_offset = dimensions as f32 * 0.1;

  draw_text_mut(&mut image, fg, horizontal_offset, vertical_offset as u32, scale, &font, &txt.trim());

  let _ = image.save(path).unwrap();
}


pub fn parse_json(filename: &str, key_name: &str) -> Value {
  
  let path = Path::new(filename);
  let mut data = File::open(&path).unwrap();
  let mut contents = String::new();
  data.read_to_string(&mut contents);

  let v: Value = serde_json::from_str(contents.as_str()).unwrap();

  v[key_name].to_owned()
}

