use linear_algebra;
use mat::{mat2x2_zero};

#[test]
pub fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
pub fn mat2x2_has_right_len() {
    let mat = mat2x2_zero();
    assert_eq!(mat.len(), 2);
    assert_eq!(mat[0].len(), 2);
}
