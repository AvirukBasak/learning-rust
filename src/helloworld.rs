use super::helperfn::get_filename;

pub fn helloworld() {
    println!("{}: Hello, world!", get_filename(file!()));
}
