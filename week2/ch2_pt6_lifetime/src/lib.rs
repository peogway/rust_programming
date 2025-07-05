fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn run_main() -> String {
    let result;

    let string1 = String::from("long string");
    let string2 = "short";

    result = longest(&string1, string2);
    result.to_string()
}