mod math;

use crate::math::Matrix;

fn main() {
    let mat = identity_matrix!(4);
    println!("{:?}", mat);
}
