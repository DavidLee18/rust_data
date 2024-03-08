pub mod vec;
pub mod queue;

use queue::Queue;

fn main() {
    let mut q = queue![1, 2, 3, 4];
    println!("{:?}", q);
    let x = q.dequeue();
    println!("{:?}", x);
    println!("{:?}", q);
}
