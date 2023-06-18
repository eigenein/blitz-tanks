use std::mem;

use futures::{stream, StreamExt, TryStream, TryStreamExt};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Merge<T> {
    Left(T),
    Right(T),
    Both(T, T),
}

pub fn merge_join_by<T, E, K: PartialEq + PartialOrd>(
    left: impl TryStream<Ok = T, Error = E> + Unpin,
    right: impl TryStream<Ok = T, Error = E> + Unpin,
    key: impl Fn(&T) -> K,
) -> impl TryStream<Ok = Merge<T>, Error = E> {
    stream::try_unfold(
        (Cursor::from(left), Cursor::from(right), key),
        |(mut left_cursor, mut right_cursor, key)| async move {
            let left = left_cursor.take().await?;
            let right = right_cursor.take().await?;

            match (left, right) {
                (None, None) => Ok(None),

                (Some(left), None) => {
                    Ok(Some((Merge::Left(left), (left_cursor, right_cursor, key))))
                }

                (None, Some(right)) => {
                    Ok(Some((Merge::Right(right), (left_cursor, right_cursor, key))))
                }

                (Some(left), Some(right)) => {
                    let left_key = key(&left);
                    let right_key = key(&right);

                    if left_key < right_key {
                        right_cursor.put_back(right);
                        Ok(Some((Merge::Left(left), (left_cursor, right_cursor, key))))
                    } else if left_key == right_key {
                        Ok(Some((Merge::Both(left, right), (left_cursor, right_cursor, key))))
                    } else {
                        left_cursor.put_back(left);
                        Ok(Some((Merge::Right(right), (left_cursor, right_cursor, key))))
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

impl<T, E, S: TryStream<Ok = T, Error = E> + Unpin> Cursor<S, T> {
    #[inline]
    async fn take(&mut self) -> Result<Option<T>, E> {
        match mem::take(&mut self.item) {
            Some(item) => Ok(item),
            None => self.stream.try_next().await,
        }
    }

    #[inline]
    fn put_back(&mut self, item: T) {
        self.item = Some(Some(item));
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::*;

    #[tokio::test]
    async fn empty_ok() {
        let result: Vec<Merge<i32>> =
            merge_join_by::<_, Infallible, _>(stream::iter([]), stream::iter([]), |_| ())
                .try_collect()
                .await
                .unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn ok() {
        let result: Vec<Merge<i32>> = merge_join_by::<_, Infallible, _>(
            stream::iter([2, 3, 4]).map(Ok),
            stream::iter([1, 2, 4]).map(Ok),
            |i| *i,
        )
        .try_collect()
        .await
        .unwrap();
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
