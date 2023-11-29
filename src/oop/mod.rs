pub mod dyn_dispatch;
pub mod numbers;
pub mod rectangle;
pub mod test;
pub mod traits;

pub fn main() {
    rectangle::main();
    numbers::main();
    traits::main();
    dyn_dispatch::main();
}
