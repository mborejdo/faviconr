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
use std::path::Path;
use std::fs::File;
use std::io::Read;
use clap::{Command, Arg};

pub fn main() {
  let matches = Command::new("faviconr")
    .author("Michael Borejdo")
    .about("generates favicons")
    .arg_required_else_help(false)
    .arg(Arg::new("config")
            .help("config")
            .long("config")
            .short('c')
            .takes_value(true)
            .required(false))
    .arg(Arg::new("offset")
            .help("horizontal offset")
            .long("offset")
            .short('h')
            .takes_value(true)
            .required(false))
    .arg(Arg::new("scale")
            .help("scale")
            .long("scale")
            .short('s')
            .takes_value(true)
            .required(false))
    .arg(Arg::new("output")
            .help("output")
            .long("output")
            .short('o')
            .takes_value(true)
            .required(false))
    .arg(Arg::new("bg")
            .help("bg")
            .long("bg")
            .takes_value(true)
            .required(false))
    .arg(Arg::new("fg")
            .help("fg")
            .long("fg")
            .takes_value(true)
            .required(false))
    .arg(Arg::new("font")
            .help("font")
            .long("font")
            .short('f')
            .takes_value(true)
            .required(false));


  let matches = matches.get_matches();
 
  let conf_file: &str = match matches.value_of("config") {
    Some(config) => {
      config
    },
    _ =>
      "./conf.json"
  };
  let output: &str = match matches.value_of("output") {
    Some(output) => {
      output
    },
    _ =>
    "./"
  };
  let fontfile: &str = match matches.value_of("font") {
    Some(font) => {
      font
    },
    _ =>
    "./assets/DejaVuSans.ttf"
  };
  let scale: f32 = match matches.value_of("scale") {
    Some(scale) => {
      scale.parse::<f32>().unwrap_or(80.0) / 100 as f32
    },
    _ =>
      0.8
  };
  let offset: u32 = match matches.value_of("offset") {
    Some(h_offset) => {
      let dimensions = 16;
      let percent_offset = h_offset.parse::<f32>().unwrap_or(0.00) / 100 as f32;
      (percent_offset * dimensions as f32) as u32
    },
    _ =>
      0
  };
  let bg: &str = match matches.value_of("bg") {
    Some(bg) => {
      bg
    },
    _ =>
    "#ff0000"
  };
  let fg: &str = match matches.value_of("fg") {
    Some(fg) => {
      fg
    },
    _ =>
    "#00ff00"
  };
  let icon_text = parse_json(conf_file, "text").to_string().replace("\"", "");
  let sizes     = parse_json(conf_file, "sizes");

  for i in 0..100 {
    if sizes[i].is_null() { break; }

    let img_size = sizes[i]["pixels"].to_string().parse::<u32>().unwrap();
    let img_format = sizes[i]["format"].to_string().replace("\"", "");

    create_favicon(
      &icon_text,
      &format!("{}favicon{}.{}", &output, &img_size, &img_format),
      img_size, 
      fontfile,
      scale, 
      offset,
      fg,
      bg
    );
  }

}

pub fn create_favicon(txt: &str, filepath: &str, dimensions: u32, fontfile: &str, scale: f32, offset: u32, fg: &str, bg: &str) {
  let black = colorsys::Rgb::from_hex_str(fg).unwrap_or(colorsys::Rgb::from((0.0, 0.0, 0.0)));
  let white = colorsys::Rgb::from_hex_str(bg).unwrap_or(colorsys::Rgb::from((255.0, 255.0, 255.0)));
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

  let font = Font::try_from_vec(std::fs::read(fontfile).unwrap()).unwrap();
  let horizontal_compression: f32 = scale;
  let vertical_compression = 0.8;
 
  let scale = Scale {
    x: (dimensions as f32) * horizontal_compression,
    y: (dimensions as f32) * vertical_compression
  };

  let horizontal_offset: u32 = offset;
  let vertical_offset = dimensions as f32 * 0.1;

  draw_text_mut(&mut image, fg, horizontal_offset, vertical_offset as u32, scale, &font, &txt.trim());

  let _ = image.save(path).unwrap();
}


pub fn parse_json(filename: &str, key_name: &str) -> Value {
  
  let path = Path::new(filename);
  let mut data = File::open(&path).unwrap();
  let mut contents = String::new();
  match data.read_to_string(&mut contents) {
      Err(e) => println!("{:?}", e),
      _ => ()
  }

  let v: Value = serde_json::from_str(contents.as_str()).unwrap();

  v[key_name].to_owned()
}

