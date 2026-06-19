#[macro_export]
macro_rules! impl_vec_of_idx {
    ($idx:ident, $range:ident, $idx_vec:ident) => {
        #[derive(Debug, Clone)]
        pub struct $idx_vec<T>(pub alloc::vec::Vec<T>);

        impl<T> core::ops::Index<$idx> for $idx_vec<T> {
            type Output = T;

            fn index(&self, index: $idx) -> &Self::Output {
                &self.0[index.0]
            }
        }

        impl<T> core::ops::IndexMut<$idx> for $idx_vec<T> {
            fn index_mut(&mut self, index: $idx) -> &mut Self::Output {
                &mut self.0[index.0]
            }
        }

        impl<T> $idx_vec<T> {
            pub fn new() -> Self {
                Self(Default::default())
            }

            pub fn new_filled(n: usize, value: impl Fn() -> T) -> Self {
                Self((0..n).map(|_| value()).collect())
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn get(&self, index: $idx) -> Option<&T> {
                self.0.get(index.0)
            }

            pub fn push(&mut self, value: T) {
                self.0.push(value);
            }

            pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> {
                self.0.iter()
            }

            pub fn enumerated_iter(&self) -> impl ExactSizeIterator<Item = ($idx, &T)> {
                self.0.iter().enumerate().map(|(i, x)| ($idx::from(i), x))
            }

            pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> {
                self.0.iter_mut()
            }

            pub fn indices(&self) -> impl Iterator<Item = $idx> {
                (0..self.0.len()).map($idx::from)
            }

            pub fn slice(&self, range: $range) -> &[T] {
                use crate::indices::IdxCore;
                let range = range.0.into_inner()..range.1.into_inner();
                &self.0[range]
            }

            pub fn as_slice(&self) -> &[T] {
                &self.0
            }
        }

        impl<T> FromIterator<T> for $idx_vec<T> {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                Self(iter.into_iter().collect())
            }
        }
    };
}
