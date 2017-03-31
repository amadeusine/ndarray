
use imp_prelude::*;
use {NdProducer, Layout};

impl_ndproducer! {
    ['a, A, D: Dimension]
    [Clone => 'a, A, D: Clone ]
    Inners {
        base,
        inner_len,
        inner_stride,
    }
    Inners<'a, A, D> {
        type Dim = D;
        type Item = ArrayView<'a, A, Ix1>;

        unsafe fn item(&self, ptr) {
            ArrayView::new_(ptr, Ix1(self.inner_len), Ix1(self.inner_stride as Ix))
        }
    }
}

pub struct Inners<'a, A: 'a, D> {
    base: ArrayView<'a, A, D>,
    inner_len: Ix,
    inner_stride: Ixs,
}


pub fn new_inners<A, D>(v: ArrayView<A, D>, axis: Axis)
    -> Inners<A, D::TrySmaller>
    where D: Dimension
{
    let ndim = v.ndim();
    let len;
    let stride;
    let iter_v;
    if ndim == 0 {
        len = 1;
        stride = 1;
        iter_v = v.try_remove_axis(Axis(0))
    } else {
        len = v.dim.last_elem();
        stride = v.strides.last_elem() as isize;
        iter_v = v.try_remove_axis(axis)
    }
    Inners {
        inner_len: len,
        inner_stride: stride,
        base: iter_v,
    }
}

impl_ndproducer! {
    ['a, A, D: Dimension]
    [Clone =>]
    InnersMut {
        base,
        inner_len,
        inner_stride,
    }
    InnersMut<'a, A, D> {
        type Dim = D;
        type Item = ArrayViewMut<'a, A, Ix1>;

        unsafe fn item(&self, ptr) {
            ArrayViewMut::new_(ptr, Ix1(self.inner_len), Ix1(self.inner_stride as Ix))
        }
    }
}

pub struct InnersMut<'a, A: 'a, D> {
    base: ArrayViewMut<'a, A, D>,
    inner_len: Ix,
    inner_stride: Ixs,
}


pub fn new_inners_mut<A, D>(v: ArrayViewMut<A, D>, axis: Axis)
    -> InnersMut<A, D::TrySmaller>
    where D: Dimension
{
    let ndim = v.ndim();
    let len;
    let stride;
    let iter_v;
    if ndim == 0 {
        len = 1;
        stride = 1;
        iter_v = v.try_remove_axis(Axis(0))
    } else {
        len = v.dim.last_elem();
        stride = v.strides.last_elem() as isize;
        iter_v = v.try_remove_axis(axis)
    }
    InnersMut {
        inner_len: len,
        inner_stride: stride,
        base: iter_v,
    }
}
