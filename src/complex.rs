#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Complex {
    pub real : f64,
    pub img : f64
}

impl Complex {
    pub fn new(real: f64 , img: f64) -> Complex {
        Complex { real: real, img: img }
      }
    pub fn mult(self, z: Self) -> Self {
        Self { real: self.real * z.real - self.img * z.img,
            img: self.real * z.img +  self.img * z.real}
    }
    pub fn add(self, z: Self) -> Self {
        Self { real: self.real + z.real, img: self.img + z.img }
    }
    fn abs(&self) -> f64{
        f64::sqrt(self.real * self.real + self.img * self.img)
    }
    pub fn indices_to_complex(x: u32, y: u32, size: u32, move_x: f64, move_y: f64, zoom: f64) -> Complex {
        Complex::new((((x as f64 - move_x * size as f64) - (2.1 * size as f64)) / zoom) as f64 / size as f64, (((y as f64 - move_y* size as f64) - (1.5 * size as f64)) / zoom) as f64 / size as f64)
    }
    pub fn check_conv(self, it: u32) -> u32 {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..=it {
          z = Complex::add(Complex::mult(z, z), self);
          if z.abs() >= 2.0 { return it - i; }
        }
        0
      }
}



#[test]
fn test1() {
    assert_eq!(Complex::mult(Complex::new(9.8, 4.3), Complex::new(3.6, 2.4)), Complex::new(24.96, 39.0));
}
#[test]
fn test2() {
    assert_eq!(Complex::add(Complex::new(9.8, 4.3), Complex::new(3.6, 2.4)), Complex::new(13.4, 6.699999999999999));
}