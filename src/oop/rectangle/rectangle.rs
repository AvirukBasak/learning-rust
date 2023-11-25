#[derive(Default)]
pub struct Rectangle {
    l: f32,
    b: f32,
}

impl Rectangle {
    pub fn new(l: f32, b: f32) -> Self {
        Self { l, b }
    }
    pub fn get_l(self: &Self) -> f32 {
        self.l
    }
    pub fn set_l(self: &mut Self, l: f32) {
        self.l = l;
    }
    pub fn get_b(self: &Self) -> f32 {
        self.b
    }
    pub fn set_b(self: &mut Self, b: f32) {
        self.b = b;
    }
    pub fn cal_area(self: &Self) -> f32 {
        self.l * self.b
    }
    pub fn cal_perimeter(self: &Self) -> f32 {
        (self.l + self.b) * 2.0
    }
}
