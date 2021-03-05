
use ndarray::prelude::*;
use ndarray::{ShapeError, ErrorKind};

#[test]
fn append_row() {
    let mut a = Array::zeros((0, 4));
    a.try_append_row(aview1(&[0., 1., 2., 3.])).unwrap();
    a.try_append_row(aview1(&[4., 5., 6., 7.])).unwrap();
    assert_eq!(a.shape(), &[2, 4]);

    assert_eq!(a,
        array![[0., 1., 2., 3.],
               [4., 5., 6., 7.]]);

    assert_eq!(a.try_append_row(aview1(&[1.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape)));
    assert_eq!(a.try_append_column(aview1(&[1.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape)));
    assert_eq!(a.try_append_column(aview1(&[1., 2.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleLayout)));
}

#[test]
fn append_row_error() {
    let mut a = Array::zeros((3, 4));

    assert_eq!(a.try_append_row(aview1(&[1.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape)));
    assert_eq!(a.try_append_column(aview1(&[1.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape)));
    assert_eq!(a.try_append_column(aview1(&[1., 2., 3.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleLayout)));
}

#[test]
fn append_row_existing() {
    let mut a = Array::zeros((1, 4));
    a.try_append_row(aview1(&[0., 1., 2., 3.])).unwrap();
    a.try_append_row(aview1(&[4., 5., 6., 7.])).unwrap();
    assert_eq!(a.shape(), &[3, 4]);

    assert_eq!(a,
        array![[0., 0., 0., 0.],
               [0., 1., 2., 3.],
               [4., 5., 6., 7.]]);

    assert_eq!(a.try_append_row(aview1(&[1.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape)));
    assert_eq!(a.try_append_column(aview1(&[1.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape)));
    assert_eq!(a.try_append_column(aview1(&[1., 2., 3.])),
        Err(ShapeError::from_kind(ErrorKind::IncompatibleLayout)));
}

#[test]
fn append_column() {
    let mut a = Array::zeros((4, 0));
    a.try_append_column(aview1(&[0., 1., 2., 3.])).unwrap();
    a.try_append_column(aview1(&[4., 5., 6., 7.])).unwrap();
    assert_eq!(a.shape(), &[4, 2]);

    assert_eq!(a.t(),
        array![[0., 1., 2., 3.],
               [4., 5., 6., 7.]]);
}
