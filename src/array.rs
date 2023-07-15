use std::ops::{Index, IndexMut};

pub struct Array<T> {
  pub size: (usize, usize),
  pub vec: Vec<T>,
}

impl<T: Clone> Array<T> {
  pub fn populate(x: usize, y: usize, c: T) -> Self {
    Self {
      size: (x, y),
      vec: (0..x * y).map(|_| c.clone()).collect(),
    }
  }
}

impl<T> Index<(usize, usize)> for Array<T> {
  type Output = T;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    self.vec.get(index.1 * self.size.0 + index.0).unwrap()
  }
}

impl<T> IndexMut<(usize, usize)> for Array<T> {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    self.vec.get_mut(index.1 * self.size.0 + index.0).unwrap()
  }
}
