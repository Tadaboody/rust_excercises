fn mean(li: &[i64]) -> f64 {
    let sum: i64 = li.iter().sum();
    sum as f64 / li.len() as f64
}

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum() {
        assert_eq!(mean(&vec![1, 2, 3]), 2.0);
        assert_eq!(mean(&vec![1, 2]), 1.5);
    }
}
