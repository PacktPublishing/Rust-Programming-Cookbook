use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    pub full_name: String,
    pub call_me: String,
    pub age: usize,
}

fn main() {
    let a_person = Person {
        full_name: "John Smith".to_owned(),
        call_me: "Smithy".to_owned(),
        age: 42,
    };
    let serialized = serde_json::to_string(&a_person).unwrap();
    println!("A serialized Person instance: {}", serialized);

    let re = Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    println!("Some regex parsing:");
    let d = "2019-01-31";
    println!("  Is {} valid? {}", d, re.captures(d).is_some());
    let d = "9999-99-00";
    println!("  Is {} valid? {}", d, re.captures(d).is_some());
    let d = "2019-1-10";
    println!("  Is {} valid? {}", d, re.captures(d).is_some());
}
