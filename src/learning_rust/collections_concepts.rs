#![allow(dead_code)]

// source: https://youtu.be/DSZqIJhkNCM?si=DGUxLLh43vOPzcsz

// -------------- IMPLEMENTATION

fn get_or_default<'a>(v: &'a Vec<&'a str>, index: usize, default: &'a str) -> &'a str {
    match v.get(index) {
        Some(value) => value,
        None => default
    }
}

struct City {
    name: String,
    population: f32,
}

impl City {
    pub fn new(name: &str, population: f32) -> City {
        City {
            name: name.to_string(),
            population,
        }
    }
}

// -------------- TESTS

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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

    #[test]
    fn vector_ownership_test() {
        // filter
        let cities = vec![
            City::new("Berlin", 3.7),
            City::new("Hamburg", 1.9),
            City::new("Munich", 1.6),
            City::new("Magdeburg", 0.260),
            City::new("Potsdam", 0.15),
        ];
        let million_cities: Vec<&City> = cities
            .iter()
            .filter(|city| city.population > 1.0)
            .collect();
        assert_eq!(million_cities.len(), 3);

        // map
        let city_names: Vec<String> = million_cities
            .iter()
            .map(|city| city.name.clone())
            .collect();
        assert_eq!(city_names, vec![
            String::from("Berlin"),
            String::from("Hamburg"),
            String::from("Munich"),
        ]);

        // reduce
        let total_population: f32 = cities
            .iter()
            .fold(0.0, |acc, city| acc + city.population);
        assert_eq!(total_population, 7.61);

        // find
        let berlin = cities
            .iter()
            .find(|city| city.name == String::from("Berlin"))
            .unwrap();
        assert_eq!(berlin.name, String::from("Berlin"));

        // max_by
        let largest = cities
            .iter()
            .max_by(|a, b| a.population.clone().partial_cmp(&b.population).unwrap())
            .unwrap();
        assert_eq!(largest.name, String::from("Berlin"));
    }

    #[test]
    fn strings_test() {
        let s1 = String::new();
        let s2 = "hello";
        let s3 = s2.to_string();
        let s4 = String::from("hello");

        assert_eq!(s1, "");
        assert_eq!(s2, s3);
        assert_eq!(s3, s4);

        let mut s5 = String::from("hello");
        s5.push_str(", world");
        s5.push('!');
        s5 += "?";
        assert_eq!(s5, "hello, world!?");

        assert_eq!("hello".to_owned() + "friend", "hellofriend");

        let s6 = "hello".to_string();
        let c1 = s6.chars().nth(0).unwrap(); // s6[0]; isn't possible because UTF-8 has characters with different lengths
        assert_eq!(c1, 'h');
    }

    #[test]
    fn hashmaps_1_test() {
        let blue = String::from("blue");
        let yellow = String::from("yellow");
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(blue.clone(), 10);
        scores.insert(yellow.clone(), 50);

        let score = scores.get(&blue);
        assert_eq!(score, Some(&10));
    }

    #[test]
    fn hashmaps_2_test() {
        let mut scores: HashMap<String, i32> = HashMap::new();
        
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 20);

        scores.entry(String::from("yellow")).or_insert(30);
        scores.entry(String::from("green")).or_insert(40);

        assert_eq!(scores.get("blue"), Some(&10));
        assert_eq!(scores.get("yellow"), Some(&20)); // not 30
        assert_eq!(scores.get("green"), Some(&40));
        assert_eq!(scores.get("red"), None);
    }

    #[test]
    fn hashmaps_3_test() {
        let text = "hello world wonderful world";
        let mut map: HashMap<&str, i32> = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        assert_eq!(map.get("hello"), Some(&1));
        assert_eq!(map.get("world"), Some(&2));
        assert_eq!(map.get("wonderful"), Some(&1));
    }
}
