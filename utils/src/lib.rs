use std::fmt::Debug;

/// Transform a vec of A to vec of B.
/// ```
/// let str_vec: Vec<&str> = vec!["a", "b", "c"];
/// let string_vec: Vec<String> = utils::vec_of(str_vec.clone());
/// assert_eq!(str_vec, string_vec);
/// ```
///
pub fn vec_of<F, T>(src: Vec<F>) -> Vec<T>
where
    F: Into<T>,
{
    src.into_iter().map(|item| item.into()).collect()
}

/// Transform a `Vec<Vec<F>>` to `Vec<Vec<T>>`.
/// ```
/// let from: Vec<Vec<&str>> = vec![vec!["a"], vec!["b", "c"]];
/// let to: Vec<Vec<String>> = utils::vec_of_vec_of(from.clone());
/// assert_eq!(from, to);
/// ```
pub fn vec_of_vec_of<F, T>(src: Vec<Vec<F>>) -> Vec<Vec<T>>
where
    F: Into<T>,
{
    src.into_iter().map(vec_of).collect()
}

/// Asserts that `lhs` and `rhs` have the same items, ignoring their orderings.
/// ```
/// let lhs = vec![1, 3, 4, 2, 2];
/// let rhs = vec![3, 2, 1, 2, 4];
/// utils::assert_same_set(lhs, rhs);
/// ```
pub fn assert_same_set<Lhs, Rhs>(mut lhs: Vec<Lhs>, rhs: Vec<Rhs>)
where
    Lhs: From<Rhs> + Ord + Debug,
{
    let mut want: Vec<Lhs> = rhs.into_iter().map(|item| item.into()).collect();
    lhs.sort_unstable();
    want.sort_unstable();
    assert_eq!(lhs, want);
}
