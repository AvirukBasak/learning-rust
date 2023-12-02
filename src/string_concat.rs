use super::helperfn::get_filename;

fn mkconcat1(s1: &str, s2: &str) -> String {
    let mut str = s1.to_owned();
    str.push(' ');
    str.push_str(s2);
    str
}

fn mkconcat2(s1: &str, s2: &str) -> String {
    s1.to_owned() + " " + s2
}

pub fn string_concat() {
    let str1 = "Hello";
    let str2 = "There";
    let s1 = String::from(str1);
    let s2 = String::from(str2);
    let s3 = mkconcat1(&s1, &s2);
    println!("{}: Result 1 = {}", get_filename(file!()), s3);
    let s3 = mkconcat2(&s1, &s2);
    println!("{}: Result 2 = {}", get_filename(file!()), s3);
}
