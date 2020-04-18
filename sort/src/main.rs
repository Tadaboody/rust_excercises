fn merge_sorted(first: &[u8], second: &[u8]) -> Vec<u8> {
    let mut first_iter = first.iter().peekable();
    let mut second_iter = second.iter().peekable();
    let size = first.len() + second.len();
    (0..size)
        .map(|_| {
            if first_iter.peek().is_some() & second_iter.peek().is_some() {
                if first_iter.peek() < second_iter.peek() {
                    *first_iter.next().unwrap()
                } else {
                    *second_iter.next().unwrap()
                }
            } else {
                *first_iter.next().xor(second_iter.next()).unwrap()
            }
        })
        .collect()
}

fn sort(v: &[u8]) -> Vec<u8> {
    if v.len() == 1 {
        return v.to_vec();
    }
    let (first, second) = v.split_at(v.len() / 2);
    merge_sorted(&sort(first), &sort(second))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort_sanity() {
        assert_eq!(crate::sort(&vec![1, 3, 2, 5, 2]), vec![1, 2, 2, 3, 5]);
    }
}
