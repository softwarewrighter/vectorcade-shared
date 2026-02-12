use approx::assert_relative_eq;
use vectorcade_shared::math::{clamp, wrap_signed_unit};

#[test]
fn clamp_works() {
    assert_relative_eq!(clamp(2.0, 0.0, 1.0), 1.0);
    assert_relative_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
    assert_relative_eq!(clamp(0.5, 0.0, 1.0), 0.5);
}

#[test]
fn wrap_unit_works() {
    assert_relative_eq!(wrap_signed_unit(1.1), -0.9);
    assert_relative_eq!(wrap_signed_unit(-1.1), 0.9);
}
