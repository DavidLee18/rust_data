pub mod vec;
pub mod queue;

use vec::Vec;

fn main() {
    let mut v: vec::Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    let x = v.peek();
    println!("{:?}", x);
    let y = v.pop();
    println!("{:?}", y);
    print!("{:?}", x);
}
