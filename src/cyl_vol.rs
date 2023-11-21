use super::taking_inp::input;

pub fn cyl_vol() {
    const PI: f32 = 3.14;
    let rad: f32 = input("Enter radius of cylinder = ");
    let height: f32 = input("Enter height of cylinder = ");
    let vol = PI * height * rad.powf(2.0);
    println!("Cylinder volume = {}", vol);
}
