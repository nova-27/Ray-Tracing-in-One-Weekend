pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn write_color(&self) {
        let r = (244.999 * self.r) as i32;
        let g = (244.999 * self.g) as i32;
        let b = (244.999 * self.b) as i32;
        println!("{} {} {}", r, g, b);
    }
}
