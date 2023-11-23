use super::helperfn::get_filename;

fn for_numbers() {
    let n1 = 43;
    let n2 = 35.5;
    let max = (n1 as f32).max(n2 as f32);
    let min = (n1 as f32).min(n2 as f32);
    println!(
        "{}: for_numbers: max = {max}, min = {min}",
        get_filename(file!())
    );
}

fn for_vectors() {
    let v1: Vec<f32> = vec![1.3, 2.0, 3.4, 51.0, 3.3, -4.0];
    let (max, min) = v1
        .iter()
        .fold((v1[0], v1[0]), |acc, &x| (acc.0.max(x), acc.1.min(x)));
    println!(
        "{}: for_vectors: max = {max}, min = {min}",
        get_filename(file!())
    );
}

pub fn min_max() {
    for_numbers();
    for_vectors();
}
