// source: https://youtu.be/DSZqIJhkNCM?si=DGUxLLh43vOPzcsz

// -------------- IMPLEMENTATION

fn get_or_default<'a>(v: &'a Vec<&'a str>, index: usize, default: &'a str) -> &'a str {
    match v.get(index) {
        Some(value) => value,
        None => default
    }
}

// -------------- TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectors_with_integers_test() {
        let a = [1, 2, 3];
        let mut v : Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        let v2 = vec![1, 2, 3];
        let v3 : Vec<i32> = a.iter().map(|x| x * 2 / 2).collect();

        assert_eq!(v, v2);
        assert_eq!(v, v3);
    }

    
    #[test]
    fn vectors_with_strings_test() {
        let v = vec!["a", "b", "c"];

        assert_eq!(get_or_default(&v, 1, "default"), "b");
        assert_eq!(get_or_default(&v, 2, "default"), "c");
        assert_eq!(get_or_default(&v, 20, "default"), "default");
    }
}
