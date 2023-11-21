pub mod area_circumference;
pub mod helloworld;
pub mod taking_inp;
pub mod interest;
pub mod cyl_vol;

fn main() {
    area_circumference::area_circumference();
    helloworld::helloworld();
    taking_inp::test();
    interest::interest();
    cyl_vol::cyl_vol();
}
