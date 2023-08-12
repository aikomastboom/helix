/// Step thru list
///
/// If the selected word is found in any of the configured lists,
/// amount steps are taken to find the next word.
pub fn increment(selected_text: &str, amount: i64, lists: &Vec<Vec<String>>) -> Option<String> {
    if selected_text.is_empty() {
        return None;
    }
    let word = selected_text;
    let lower_case_word: String = word.to_lowercase();
    for (_i, walkway) in lists.iter().enumerate() {
        for (index, w) in walkway.iter().enumerate() {
            if !w.is_empty() && lower_case_word.eq(w.to_lowercase().as_str()) {
                let mut is_uppercase: bool = false;
                let mut is_capitalized: bool = false;
                if word.eq(w.to_uppercase().as_str()) {
                    is_uppercase = true;
                } else {
                    is_capitalized = word.chars().next().unwrap().is_uppercase();
                }
                return Some(increment_walkway(
                    walkway,
                    index,
                    is_capitalized,
                    is_uppercase,
                    amount,
                ));
            }
        }
    }
    None
}

fn increment_walkway(
    walkway: &Vec<String>,
    index: usize,
    is_capitalized: bool,
    is_uppercase: bool,
    amount: i64,
) -> String {
    let pos: usize = (index as i64 + amount).rem_euclid(walkway.len() as i64) as usize;
    let s: String = walkway.get(pos).unwrap().into();
    if is_uppercase {
        s.to_uppercase()
    } else if is_capitalized {
        // https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
        s.chars().next().unwrap().to_uppercase().to_string()
            + s.chars().skip(1).collect::<String>().as_str()
    } else {
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_increment() {
        let walkway_one = vec!["true".to_owned(), "false".to_owned()];
        let walkway_two = vec!["ja".to_owned(), "nee".to_owned()];
        let config = vec![walkway_two, walkway_one];

        let tests = [
            ("false", 2, "false"),
            ("false", 1, "true"),
            ("false", -1, "true"),
            ("false", -2, "false"),
            ("False", 1, "True"),
            ("True", -1, "False"),
            ("Ja", 3, "Nee"),
            ("JA", 3, "NEE"),
        ];

        for (original, amount, expected) in tests {
            assert_eq!(increment(original, amount, &config).unwrap(), expected);
        }
    }
}
