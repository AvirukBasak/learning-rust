#[derive(Default)]
pub struct Rectangle {
    l: f32,
    b: f32,
}

impl Rectangle {
    pub fn new(l: f32, b: f32) -> Self {
        Self { l, b }
    }
    pub fn get_l(&self) -> f32 {
        self.l
    }
    pub fn set_l(&mut self, l: f32) {
        self.l = l;
    }
    pub fn get_b(&self) -> f32 {
        self.b
    }
    pub fn set_b(&mut self, b: f32) {
        self.b = b;
    }
    pub fn cal_area(&self) -> f32 {
        self.l * self.b
    }
    pub fn cal_perimeter(&self) -> f32 {
        (self.l + self.b) * 2.0
    }
}
