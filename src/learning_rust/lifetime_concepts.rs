#![allow(dead_code)]

// source: https://www.youtube.com/watch?v=juIINGuZyBc

// -------------- IMPLEMENTATION

// the generics annotation <'a> tells the compiler the lifetime of returned value is equal to the smallest of x and y
pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if y.len() > x.len() {
        y
    } else {
        x
    }
}

pub fn take_first_ignore_second<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

pub fn concatenate_strings(x: &str, y: &str) -> String {
    let result = format!("{}{}", x, y);
    // return &result (as &str) is not allowed, because result is created inside this scope and will be discarded if only reference is returned
    result
}

pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // in this case, lifetime of &self is taken
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// without generics also works, because the compiler is able to infer the lifetime in certain cases
// pub fn first_word(s: &str) -> &str {
pub fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// -------------- TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_test() {
        // equal lifetimes
        let s1 = String::from("hello");
        let s2 = String::from("hello longer");
        let result = longest_string(s1.as_str(), s2.as_str());
        assert_eq!(result, s2);

        // different lifetimes
        let s1 = String::from("hello");
        {
            let s2 = String::from("hello longer");
            let other_result = longest_string(s1.as_str(), s2.as_str());
            assert_eq!(other_result, s2);
        }
    }

    #[test]
    fn take_first_ignore_second_test() {
        let s1 = String::from("hello");
        let other_result: &str;

        {
            let s2 = String::from("hello longer");
            // the lifetime of s2 is ignored, as take_first_ignore_second has only generics covering the first argument
            other_result = take_first_ignore_second(s1.as_str(), s2.as_str());
        }
        assert_eq!(other_result, s1);
    }

    #[test]
    fn concatenate_strings_test() {
        let s1 = String::from("hello ");
        let s2 = String::from("world");
        let result = concatenate_strings(&s1, &s2);
        assert_eq!(result, String::from("hello world"));
    }

    #[test]
    fn important_excerpt_test() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find ");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        assert_eq!(i.return_part("Attention please"), first_sentence);
    }

    #[test]
    fn first_word_test() {
        let s1 = String::from("hello world");
        let first = first_word(&s1);
        assert_eq!(first, String::from("hello"));
    }

    #[test]
    fn static_lifetime_test() {
        // when set lifetime with 'static, it keeps lifetime while program is running. String literals are always that case
        let s: &'static str = "hello";

        assert_eq!(s, String::from("hello"));
    }
}