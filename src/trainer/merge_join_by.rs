use std::mem;

use futures::{stream, Stream, StreamExt};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Merge<T> {
    Left(T),
    Right(T),
    Both(T, T),
}

fn merge_join_by<T, K: PartialEq + PartialOrd>(
    left: impl Stream<Item = T> + Unpin,
    right: impl Stream<Item = T> + Unpin,
    key: impl Fn(&T) -> K,
) -> impl Stream<Item = Merge<T>> {
    stream::unfold(
        (Cursor::from(left), Cursor::from(right), key),
        |(mut left_cursor, mut right_cursor, key)| async move {
            let left = left_cursor.take().await;
            let right = right_cursor.take().await;

            match (left, right) {
                (None, None) => None,

                (Some(left), None) => Some((Merge::Left(left), (left_cursor, right_cursor, key))),

                (None, Some(right)) => {
                    Some((Merge::Right(right), (left_cursor, right_cursor, key)))
                }

                (Some(left), Some(right)) => {
                    let left_key = key(&left);
                    let right_key = key(&right);

                    if left_key < right_key {
                        right_cursor.put_back(right);
                        Some((Merge::Left(left), (left_cursor, right_cursor, key)))
                    } else if left_key == right_key {
                        Some((Merge::Both(left, right), (left_cursor, right_cursor, key)))
                    } else {
                        left_cursor.put_back(left);
                        Some((Merge::Right(right), (left_cursor, right_cursor, key)))
                    }
                }
            }
        },
    )
}

struct Cursor<S, T> {
    item: Option<Option<T>>,
    stream: S,
}

impl<S, T> From<S> for Cursor<S, T> {
    fn from(stream: S) -> Self {
        Self { stream, item: None }
    }
}

impl<T, S: Stream<Item = T> + Unpin> Cursor<S, T> {
    #[inline]
    async fn take(&mut self) -> Option<T> {
        match mem::take(&mut self.item) {
            Some(item) => item,
            None => self.stream.next().await,
        }
    }

    #[inline]
    fn put_back(&mut self, item: T) {
        self.item = Some(Some(item));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn empty_ok() {
        let result: Vec<Merge<i32>> =
            merge_join_by(stream::iter([]), stream::iter([]), |_| ()).collect().await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn ok() {
        let result: Vec<Merge<i32>> =
            merge_join_by(stream::iter([2, 3, 4]), stream::iter([1, 2, 4]), |i| *i)
                .collect()
                .await;
        assert_eq!(
            result,
            [
                Merge::Right(1),
                Merge::Both(2, 2),
                Merge::Left(3),
                Merge::Both(4, 4)
            ]
        );
    }
}
