pub fn mean(li: &[i64]) -> f64 {
    let sum: i64 = li.iter().sum();
    sum as f64 / li.len() as f64
}

use std::cmp::Ord;

pub fn med<T: Ord + Copy>(list: &[T]) -> T {
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

pub fn mode<T: Eq + Copy + Hash + Ord>(it: &[T]) -> T {
    let counter = count(it);
    let mut pairs: Vec<(&T, &i64)> = counter.iter().collect();
    pairs.sort_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap());
    let (key, _) = pairs.last().unwrap();
    **key
}

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn to_pig_latin(word: &mut String) {
    let vowels: HashSet<char> = HashSet::from_iter("aeiou".chars());
    if !vowels.contains(&word.char_indices().next().unwrap().1.to_ascii_lowercase()) {
        let ch = word.remove(0);
        word.push_str(&format!("-{}ay", ch.to_string()));
    } else {
        word.push_str("-hay");
    }
}

fn sorted<T: Ord + Clone>(collection: &[T]) -> Vec<T> {
    let mut copy = collection.to_owned();
    copy.sort();
    copy
}

use regex::Regex;
use std::collections::BTreeMap;
// We use btree and not hashmap to keep the keys sorted.
type Employees = BTreeMap<String, Vec<String>>;
// Instead of printing I return a string with what should be printed.
pub fn process_command(employees: &mut Employees, command: &str) -> Option<String> {
    let add_command = Regex::new(r"(?i)add (?P<name>\w+) to (?P<department>\w+)").unwrap();
    let list_command = Regex::new(r"(?i)list(?:\s(?P<department>\w+))?").unwrap();
    if let Some(add) = add_command.captures(&command.to_ascii_lowercase()) {
        employees
            .entry(add.name("department").unwrap().as_str().to_owned())
            .or_default()
            .push(add.name("name").unwrap().as_str().to_owned());
        None
    } else if let Some(list_match) = list_command.captures(command) {
        fn format_department(department: &str, names: &[String]) -> String {
            format!("{}: {}", department, sorted(&names).join(", "))
        };

        println!("{:?}", list_match);
        if let Some(department) = list_match.name("department") {
            employees
                .get(&department.as_str().to_lowercase())
                .map(|names| format_department(department.as_str(), names))
        } else {
            Some(
                employees
                    .iter()
                    // unpack the tuple becuse I don't know how to starmap
                    .map(|(department, names)| format_department(department, names))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        }
    } else {
        panic!("Unkown command! AHHHHHH!")
    }
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

    #[test]
    fn pig_latin_sanity() {
        let mut a = "first".to_owned();
        to_pig_latin(&mut a);
        assert_eq!(a, "irst-fay");
        let mut b = "apple".to_owned();
        to_pig_latin(&mut b);
        assert_eq!(b, "apple-hay");
    }

    #[test]
    fn add_command() {
        let mut employees: Employees = Employees::new();
        process_command(&mut employees, "add Susan to State");
        process_command(&mut employees, "add John to State");
        assert!(employees.get("state").unwrap().iter().any(|e| e == "susan"));
        process_command(&mut employees, "add Michael to Finance");
        assert!(employees
            .get("finance")
            .unwrap()
            .contains(&"michael".to_owned()));
    }

    #[test]
    fn list_command() {
        let mut employees: Employees = Employees::new();
        process_command(&mut employees, "add Susan to State");
        process_command(&mut employees, "add John to State");
        assert_eq!(
            process_command(&mut employees, "list"),
            Some("state: john, susan".to_owned())
        );
        process_command(&mut employees, "add Michael to Finance");
        assert_eq!(
            process_command(&mut employees, "list"),
            Some("finance: michael\nstate: john, susan".to_owned())
        );
    }
    #[test]
    fn list_department_command() {
        let mut employees: Employees = Employees::new();
        process_command(&mut employees, "add Susan to State");
        process_command(&mut employees, "add John to State");
        process_command(&mut employees, "add Michael to Finance");
        assert_eq!(
            process_command(&mut employees, "list State").map(|st| st.to_lowercase()),
            Some("state: john, susan".to_owned())
        );
    }
}
