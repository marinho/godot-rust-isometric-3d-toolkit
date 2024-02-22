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

    #[test]
    fn vector_ownership_test() {
        // filter
        let cities = vec![
            City{ name: String::from("Berlin"), population: 3.7 },
            City{ name: String::from("Hamburg"), population: 1.9 },
            City{ name: String::from("Munich"), population: 1.6 },
            City{ name: String::from("Magdeburg"), population: 0.260 },
            City{ name: String::from("Potsdam"), population: 0.15 },
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
}
