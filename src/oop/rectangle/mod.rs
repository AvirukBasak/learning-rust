pub mod rectangle;

use crate::helperfn::get_filename;
use crate::helperfn::input;

pub fn main() {
    let l: f32 = input(&format!("{}: Enter length: ", get_filename(file!())));
    let b: f32 = input(&format!("{}: Enter breadth: ", get_filename(file!())));
    let rect = rectangle::Rectangle::new(l, b);
    let a = rect.cal_area();
    let p = rect.cal_perimeter();
    println!(
        "{}: Dim = ({}, {}), Area = {a}, Perimeter = {p}",
        get_filename(file!()),
        rect.get_l(),
        rect.get_b()
    );
    let rect = rectangle::Rectangle::default();
    let a = rect.cal_area();
    let p = rect.cal_perimeter();
    println!(
        "{}: Dim = ({}, {}), Area = {a}, Perimeter = {p}",
        get_filename(file!()),
        rect.get_l(),
        rect.get_b()
    );
}
