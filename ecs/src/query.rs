use std::ops::Deref;

pub struct Query<'a, T> {
    query: &'a [T]
}

impl<T> Deref for Query<'_, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.query
    }
}

impl<'a, T> IntoIterator for Query<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.query.into_iter()
    }
}