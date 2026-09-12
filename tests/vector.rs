// The implementation of `vector_type!` should not rely on anything in the prelude
#![no_implicit_prelude]

extern crate vector_type;
extern crate core;

use vector_type::vector_type;

vector_type!{
  struct Vector;

  enum VectorField as u8;

  abstract {
    x: X,
    y: Y,
    z: Z
  }
}

const _: () = {
  core::assert!(VectorField::VARIANTS_COUNT == 3);
};

#[test]
fn main() {
  let value: Vector<usize> = Vector { x: 1, y: 2, z: 3 };

  core::assert_eq!(*value.get(VectorField::X), 1);
  core::assert_eq!(*value.get(VectorField::Y), 2);
  core::assert_eq!(*value.get(VectorField::Z), 3);

  core::assert_eq!(value.into_array(), [1, 2, 3]);
}
