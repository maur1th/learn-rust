extern crate playground;
use playground::collections::vector;

fn main() {
    println!("{:?}", vector::get_properties(&vec![2, 1, 3, 2]));
    println!("{:?}", vector::get_properties(&vec![]));
    println!("{:?}", vector::get_properties(&vec![2, 1, 3, 3, 3]));
}
