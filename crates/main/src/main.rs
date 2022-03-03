use std::{
  fs::File,
  io::{BufWriter, Read},
};
mod paint;
extern crate image;

use css_parser;
use html_parser;

fn main() {
  // Parse command-line options:
  // let mut opts = getopts::Options::new();
  // opts.optopt("h", "html", "HTML document", "FILENAME");
  // opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
  // opts.optopt("o", "output", "Output file", "FILENAME");
  // opts.optopt("f", "format", "Output file format", "png | pdf");

  // let matches = opts.parse(std::env::args().skip(1)).unwrap();
  // let str_arg = |flag: &str, default: &str| -> String {
  //     matches.opt_str(flag).unwrap_or(default.to_string())
  // };

  // // Choose a format:
  // let png = match &str_arg("f", "png")[..] {
  //     "png" => true,
  //     "pdf" => false,
  //     x => panic!("Unknown output format: {}", x),
  // };

  // Read input files:
  let html = read_source("example/test.html".to_string());
  let css = read_source("example/test.css".to_string());

  // Since we don't have an actual window, hard-code the "viewport" size.
  let mut viewport: layout::Dimensions = Default::default();
  viewport.content.width = 800.0;
  viewport.content.height = 600.0;

  // Parsing and rendering:
  let root_node = html_parser::parse(html);
  let stylesheet = css_parser::parse(css);
  let style_root = style::style_tree(&root_node, &stylesheet);
  let layout_root = layout::layout_tree(&style_root, viewport);
  // println!(">>>>>>>>{:#?}", layout_root);
  // Create the output file:
  // let filename = str_arg("o", if png { "output.png" } else { "output.pdf" });
  let mut file = BufWriter::new(File::create("output.png").unwrap());

  // Write to the file:
  let ok = {
    let canvas = paint::paint(&layout_root, viewport.content);
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
      let color = canvas.pixels[(y * w + x) as usize];
      image::Pixel::from_channels(color.r, color.g, color.b, color.a)
    });
    image::ImageRgba8(img).save(&mut file, image::PNG).is_ok()
  };
  if ok {
    println!("Saved output as {}", "output")
  } else {
    println!("Error saving output as {}", "output")
  }
}

fn read_source(filename: String) -> String {
  let mut str = String::new();
  File::open(filename)
    .unwrap()
    .read_to_string(&mut str)
    .unwrap();
  str
}

#[test]
fn test2() {
  use insta::assert_debug_snapshot;
  let html = read_source("example/test.html".to_string());
  let css = read_source("example/test.css".to_string());

  // Since we don't have an actual window, hard-code the "viewport" size.
  let mut viewport: layout::Dimensions = Default::default();
  viewport.content.width = 800.0;
  viewport.content.height = 600.0;

  // Parsing and rendering:
  let root_node = html_parser::parse(html);
  let stylesheet = css_parser::parse(css);
  let style_root = style::style_tree(&root_node, &stylesheet);
  assert_debug_snapshot!(style_root);
  // let layout_root = layout::layout_tree(&style_root, viewport);
  // assert_debug_snapshot!(layout_root);
}


#[test]
fn test3() {
  use insta::assert_debug_snapshot;
  let html = read_source("example/test.html".to_string());
  let css = read_source("example/test.css".to_string());

  // Since we don't have an actual window, hard-code the "viewport" size.
  let mut viewport: layout::Dimensions = Default::default();
  viewport.content.width = 800.0;
  viewport.content.height = 600.0;

  // Parsing and rendering:
  let root_node = html_parser::parse(html);
  let stylesheet = css_parser::parse(css);
  let style_root = style::style_tree(&root_node, &stylesheet);
  // assert_debug_snapshot!(style_root);
  let layout_root = layout::layout_tree(&style_root, viewport);
  assert_debug_snapshot!(layout_root);
}