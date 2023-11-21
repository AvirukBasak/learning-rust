use super::taking_inp;

pub fn interest() {
    let prin: f32 = taking_inp::input("Enter principal (Rs): ");
    let time: f32 = taking_inp::input("Enter period (y): ");
    let rate: f32 = taking_inp::input("Enter rate (%): ");
    let si = prin * time * rate / 100.0;
    println!("si = {}", si);
}
