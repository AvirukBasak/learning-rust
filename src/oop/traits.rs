use crate::helperfn::get_filename;
use crate::helperfn::input;

mod traits {

    pub trait Shape {
        fn area(&self) -> f32;
        fn perimeter(&self) -> f32;
    }

    pub trait ShapeConsts {
        const PI: f32;
    }

    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f32 {
            Self::PI * self.radius * self.radius
        }

        fn perimeter(&self) -> f32 {
            2.0 * Self::PI * self.radius
        }
    }

    impl ShapeConsts for Circle {
        const PI: f32 = std::f32::consts::PI;
    }
}

/// Static polymorphism with traits
fn cal_area_and_peri<T>(shape: &T) -> (f32, f32)
where
    T: traits::Shape,
{
    (shape.area(), shape.perimeter())
}

pub fn main() {
    let radius: f32 = input(&format!("{}: Enter radius: ", get_filename(file!())));
    let circle = traits::Circle::new(radius);
    let (a, p) = cal_area_and_peri(&circle);
    println!("{}: Area = {}, Perimeter = {}", get_filename(file!()), a, p);
}
