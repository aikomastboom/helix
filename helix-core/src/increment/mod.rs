mod date_time;
mod integer;
mod list;

pub fn integer() -> Box<dyn Fn(&str, i64) -> Option<String>> {
    Box::new(move |selected_text: &str, amount: i64| integer::increment(selected_text, amount))
}

pub fn date_time() -> Box<dyn Fn(&str, i64) -> Option<String>> {
    Box::new(move |selected_text: &str, amount: i64| date_time::increment(selected_text, amount))
}

pub fn list(config: &Vec<Vec<String>>) -> Box<dyn Fn(&str, i64) -> Option<String> + '_> {
    Box::new(move |selected_text: &str, amount: i64| list::increment(selected_text, amount, config))
}
