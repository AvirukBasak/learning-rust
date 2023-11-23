use super::helperfn::get_filename;
use super::helperfn::input;

/// Valid denominations are as follows
/// 500, 200, 100, 50, 20, 10, 5, 2, 1
fn min_denom(sum: i32) -> i32 {
    match sum {
        ..=0 => 0,
        500 | 200 | 100 | 50 | 20 | 10 | 5 | 2 | 1 => 1,
        _ => {
            1 + vec![
                min_denom(sum - 500),
                min_denom(sum - 200),
                min_denom(sum - 100),
                min_denom(sum - 50),
                min_denom(sum - 20),
                min_denom(sum - 10),
                min_denom(sum - 5),
                min_denom(sum - 2),
                min_denom(sum - 1),
            ]
            .iter()
            .fold(std::i32::MAX, |acc, &x| acc.min(x))
        }
    }
}

pub fn min_denominations() {
    let sum: i32 = input(&format!("{}: Enter sum: ", get_filename(file!())));
    println!("{}: Result = {}", get_filename(file!()), min_denom(sum));
}
