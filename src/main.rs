pub mod area_circumference;
pub mod cyl_vol;
pub mod division;
pub mod helloworld;
pub mod helperfn;
pub mod interest;
pub mod string_concat;
pub mod taking_inp;
pub mod min_max;

fn main() {
    area_circumference::area_circumference();
    helloworld::helloworld();
    taking_inp::test();
    interest::interest();
    cyl_vol::cyl_vol();
    string_concat::string_concat();
    division::division();
    min_max::min_max();
}
