use itertools::{Itertools, MultiProduct};

pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
    where
        I: Iterator + Clone,
        I::Item: Clone {
    std::iter::repeat(it)
        .take(repeat)
        .multi_cartesian_product()
}

