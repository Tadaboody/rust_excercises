fn mean(li: &[i64]) -> f64 {
    let sum: i64 = li.iter().sum();
    sum as f64 / li.len() as f64
}

use std::cmp::Ord;

fn med<T: Ord + Copy>(list: &[T]) -> T {
    let mut new_vec = list.to_vec();
    new_vec.sort();
    new_vec[new_vec.len() / 2]
}
use std::collections::HashMap;
use std::hash::Hash;

fn count<T: Eq + Copy + Hash>(it: &[T]) -> HashMap<T, i64> {
    let mut result: HashMap<T, i64> = HashMap::new();
    for thing in it {
        *result.entry(*thing).or_insert(0) += 1;
    }
    result
}

fn mode< T: Eq + Copy + Hash + Ord>(it: &[T]) -> T {
    let counter = count(it);
    let mut pairs: Vec<(&T, &i64)> = counter.iter().collect();
    pairs.sort_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap());
    let (key, _) = pairs.last().unwrap();
    **key
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mean_sanity() {
        assert_eq!(mean(&vec![1, 2, 3]), 2.0);
        assert_eq!(mean(&vec![1, 2]), 1.5);
    }
    #[test]
    fn median_sanity() {
        assert_eq!(med(&vec![1, 2, 3]), 2);
        assert_eq!(med(&vec![1, 2]), 2);
        assert_eq!(med(&vec!["a", "b", "c"]), "b");
    }

    #[test]
    fn mode_sanity() {
        assert_eq!(mode(&vec![1, 1, 2, 2, 1, 3]), 1);
        assert_eq!(mode(&vec!["a", "b", "b", "c"]), "b")
    }
}
