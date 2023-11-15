use std::fs::File;
use std::io::prelude::*;


#[derive(Copy, Clone)]
pub struct Pixel {
  r: u8,
  g: u8,
  b: u8
}

impl Pixel {
  pub fn new(r: u8 , g: u8, b: u8) -> Pixel {
      Pixel{r: r, g: g, b: b}
    }
  pub fn new_from_vector(v: Vec<i32>) -> Pixel {
    Pixel { r: v[0] as u8, g: v[1] as u8, b: v[2] as u8 }
  }
  pub fn to_string(&self) -> String {
    let r = self.r.to_string();
    let g = self.g.to_string();
    let b = self.b.to_string();
    format!("{r} {g} {b} ")
  }
  pub fn new_black() -> Pixel {
    Pixel{r: 0, g: 0, b: 0}
  }
  pub fn new_white() -> Pixel {
    Pixel{r: 255, g: 255, b: 255}
  }
  pub fn set(&mut self, r: u8, g: u8, b: u8)  {
    self.r = r;
    self.g = g;
    self.b = b;
  }
  pub fn add(&self, color_change: Pixel, times: u32) -> Pixel{
    Pixel::new( ((self.r as u32 + (color_change.r as u32) * times) % 255) as u8,
                ((self.g as u32 + (color_change.g as u32) * times) % 255) as u8,
                ((self.b as u32 + (color_change.b as u32) * times) % 255) as u8)
  }
}

pub struct Image {
  height: u32,
  width: u32,
  v: Vec<Vec<Pixel>>
}

impl Image {
  pub fn new(height: u32, width: u32, v: Vec<Vec<Pixel>>) -> Image {
    Image{height: height, width: width, v: v}
  }
  pub fn new_blank(height: u32, width: u32, color: Pixel) -> Image {
    let v = vec![vec![color; width as usize]; height as usize];
    Image { height: height, width: width, v: v}
  }
  pub fn get_size(&self) -> (u32, u32) {
    (self.height, self.width)
  }
  pub fn write_to_ppm(&self,filename: &str)-> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(&mut file, "P3\n{} {}\n255\n", self.width, self.height)?;
    let y = self.v.iter().map(|row| 
        row.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ")).collect::<Vec<String>>().join("\n");
      writeln!(&mut file, "{}", y)
    }
    pub fn to_string(&self) -> String {
      self.v.iter().map(|row| 
        row.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ")).collect::<Vec<String>>().join("\n")
    }
    pub fn paint_pixel(&mut self, x: u32, y: u32, color: Pixel) -> () {
      self.v[y as usize][x as usize] = color;
    }

}
