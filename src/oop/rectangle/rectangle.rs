pub struct Rectangle {
    l: f32,
    b: f32,
}

impl Rectangle {
    pub fn new(l: f32, b: f32) -> Rectangle {
        Rectangle { l, b }
    }
    pub fn cal_area(self: &Self) -> f32 {
        self.l * self.b
    }
    pub fn cal_perimeter(self: &Self) -> f32 {
        (self.l + self.b) * 2.0
    }
}
