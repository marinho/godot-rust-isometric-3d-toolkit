#![allow(dead_code)]

// source: https://www.youtube.com/watch?v=VFIOSWy93H0

// -------------- IMPLEMENTATION

pub fn copy_and_move_ownership() {
    let x = 5;
    let _y = x;

    let s1 = String::from("hello"); // this is invalidated when line below moves ownership of this value to s2
    let _s2 = s1;
}

pub fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

pub fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

pub fn gives_ownership() -> String {
    let some_string =  String::from("hello");
    some_string
}

pub fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

pub fn calculate_length_1(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

pub fn calculate_length_2(s: &String) -> usize {
    let length = s.len();
    length
}

pub fn change_a_string(some_string: &mut String) {
    // if some_string: &mut String, it's now allowed, because some_string is just a borrowed reference
    some_string.push_str(", world");
}

// This example is not allowed, because just reference of s is returned, ownership is not given, so s is discarded before it's returned
// pub fn dangle_reference() -> &String {
//     let s = String::from("hello");
//     &s
// }

pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return s;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// -------------- TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn copy_and_move_ownership_test() {
        copy_and_move_ownership();
        assert!(true);
    }

    #[test]
    fn takes_ownership_test() {
        let s = String::from("hello");
        takes_ownership(s);
        // println!("Post takes_ownership: {}", s); // not allowed, since ownership of "s" has been passed to takes_ownership
    }

    #[test]
    fn makes_copy_test() {
        let x = 5;
        makes_copy(x);
        assert_eq!(x, 5); // allowed, because i32 is copied, differently than strings
    }

    #[test]
    fn gives_ownership_test() {
        let s1 = gives_ownership();
        assert_eq!(s1, String::from("hello"));
    }

    #[test]
    fn takes_and_gives_back_test() {
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // println!("s2: {}", s2); // now allowed, because ownership has been moved to takes_and_gives_back, then to s3
        assert_eq!(s3, String::from("hello"));
    }

    #[test]
    fn calculate_length_1_test() {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length_1(s1);
        assert_eq!(s2, String::from("hello"));
        assert_eq!(len, 5);
    }

    #[test]
    fn calculate_length_2_test() {
        let s1 = String::from("hello");
        let len = calculate_length_2(&s1); // using reference
        assert_eq!(s1, String::from("hello")); // allowed because s1 didn't pass ownership, but just borrowed reference
        assert_eq!(len, 5);
    }

    #[test]
    fn change_a_string_test() {
        let mut s1 = String::from("hello");
        change_a_string(&mut s1);
        assert_eq!(s1, String::from("hello, world"));
    }

    #[test]
    fn imutable_vs_mutable_test() {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s; // not allowed after immutable references are given above

        assert_eq!(*r1, String::from("hello"));
        assert_eq!(*r2, String::from("hello"));

        let r3 = &mut s; // allowed because r1 and r2 have been discarded after above lines
        assert_eq!(*r3, String::from("hello"));
    }

    #[test]
    fn first_word_test() {
        let s1 = String::from("hello world");
        let word1 = first_word(&s1);
        assert_eq!(word1, &s1[0..5]);

        let mut s2 = String::from("hello");
        let word2 = first_word(&s2);
        assert_eq!(word2, String::from("hello"));

        s2.clear();
        assert_eq!(s2, String::from(""));
    }
}
