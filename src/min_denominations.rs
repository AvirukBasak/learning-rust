use super::helperfn::get_filename;
use super::helperfn::input;

fn min_denom(dens: &Vec<i32>, sum: i32) -> i32 {
    if sum == 0 {
        return 0;
    }
    let mut res = std::i32::MAX;
    for den in dens.iter() {
        if den > &sum {
            continue;
        }
        let ret = res.min(1 + min_denom(dens, sum - den));
        if ret != std::i32::MAX {
            res = ret;
        }
    }
    return res;
}

pub fn min_denominations() {
    let sum: i32 = input(&format!("{}: Enter sum: ", get_filename(file!())));
    let denominations = vec![25, 10, 5];
    let result = min_denom(&denominations, sum);
    println!("{}: Result = {result}", get_filename(file!()));
}
