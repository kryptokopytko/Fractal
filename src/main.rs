use complex::Complex;
use image::Image;
use image::Pixel;
use std::io;
pub mod complex;
pub mod image;

fn mandelbrot(size: u32, it: u32, zoom: f64, move_x: f64, move_y: f64, base_color: Pixel, color_change: Pixel) -> Image {
  let mut img = Image::new_blank(3 * size, 3 * size, Pixel::new_white());
  for x in 0..size * 3 {
    for y in 0..size * 3 {
      let c = Complex::indices_to_complex(x, y, size, move_x, move_y, zoom).check_conv(it);
      img.paint_pixel(x, y, base_color.add(color_change, c));
    }
  }
  img
}

fn cin() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("failed to readline");
  input
}
fn string_to_int(string: String) -> i32 {
  string.trim().parse::<i32>().unwrap()
}

fn main() {
  println!("Are you ready to generate a fractal?\nEnter size (100 for little, 2000 for large picture): ");
  let size = string_to_int(cin());
  println!("Do you want your fractal to be zoomed, moved left or up?/nEnter 3 floating point values (1.0 0.0 0.0 for standard): ");
  let zoom = cin().split_whitespace().collect::<Vec<&str>>().iter().map(|s| s.to_string().trim().parse::<f64>().unwrap()).collect::<Vec<f64>>();
  println!("How many iterations do you want (5 for rough strokes, 10 for pretty result, 30 for a lot of detail: ");
  let it = string_to_int(cin());
  println!("What base color do you want(r g b)? (example: 255 0 0): ");
  let base_color = Pixel::new_from_vector(cin().split_whitespace().collect::<Vec<&str>>().iter().map(|s| string_to_int(s.to_string())).collect::<Vec<i32>>());
  println!("What color change do you want (r g b)? (example: 40 0 0): ");
  let color_change = Pixel::new_from_vector(cin().split_whitespace().collect::<Vec<&str>>().iter().map(|s| string_to_int(s.to_string())).collect::<Vec<i32>>());
  let img = mandelbrot(size as u32, it as u32, zoom[0], zoom[1],zoom[2], base_color, color_change);
  assert!(img.write_to_ppm("test.ppm").is_ok());
}

#[test]
fn test1() {
  let color = Pixel::new(255, 0, 0);
  let ccolor = Pixel::new(40, 0, 0);
  let img = mandelbrot(1000, 10, 1.0, 0.0, 0.0, color, ccolor);
  assert!(img.write_to_ppm("standard.ppm").is_ok());
}

#[test]
fn test2() {
  let color = Pixel::new(0, 0, 0);
  let ccolor = Pixel::new(0, 20, 20);
  let img = mandelbrot(1000, 15, 1.0, 0.0, 0.0, color, ccolor);
  assert!(img.write_to_ppm("mint.ppm").is_ok());
}

#[test]
fn test3() {
  let color = Pixel::new(0, 0, 0);
  let ccolor = Pixel::new(0, 40, 20);
  let img = mandelbrot(1000, 50, 1.0, 0.0, 0.0, color, ccolor);
  assert!(img.write_to_ppm("sharp.ppm").is_ok());
}

#[test]
fn test4() {
  let color = Pixel::new(0, 0, 0);
  let ccolor = Pixel::new(90, 20, 100);
  let img = mandelbrot(1500, 20, 4.0, 1.0, 3.0, color, ccolor);
  assert!(img.write_to_ppm("fragment.ppm").is_ok());
}

#[test]
fn test5() {
  let color = Pixel::new(255, 255, 255);
  let ccolor = Pixel::new(50, 50, 0);
  let img = mandelbrot(1500, 5, 1.0, 0.0, 0.0, color, ccolor);
  assert!(img.write_to_ppm("rough.ppm").is_ok());
}