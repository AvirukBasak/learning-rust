use super::helperfn::get_filename;

pub fn area_circumference() {
    let rad = 3.0;
    const PI: f32 = std::f32::consts::PI;
    let area = PI * (rad as f32).powf(2.0);
    let circum = 2.0 * PI * rad;
    println!("{}: Area = {}", get_filename(file!()), area);
    println!("{}: Circumference = {}", get_filename(file!()), circum);
}
