#![allow(dead_code)]

// source: https://youtu.be/DSZqIJhkNCM?si=DGUxLLh43vOPzcsz

// -------------- IMPLEMENTATION
pub enum PersonAddress {
    Street { name: String, number: u16 },
    PostalCode(u16),
    Place(String),
    IpAddress(u8, u8, u8, u8),
}

impl PersonAddress {
    pub fn get_address(&self) -> String {
        match self {
            PersonAddress::Street { name, number } => format!("{} {}", name, number),
            PersonAddress::PostalCode(code) => format!("{}", code),
            PersonAddress::Place(place) => format!("{}", place),
            PersonAddress::IpAddress(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
        }
    }
}

// #[derive(Debug)]
pub enum GermanyState {
    Hamburg,
    Bremen,
    Berlin,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(GermanyState),
}

impl Coin {
    pub fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                match state {
                    GermanyState::Berlin => 24,
                    _ => 25, // all other state options
                }
            },
        }
    }
}

pub fn plus_one(value: Option<i32>) -> Option<i32> {
    if let Some(0) = value {
        return None;
    }

    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// -------------- TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_type_test() {
        let street = PersonAddress::Street{ name: String::from("Main Street"), number: 46};
        let postal_code = PersonAddress::PostalCode(13022);
        let place = PersonAddress::Place(String::from("Alexander Platz"));
        let ip_address = PersonAddress::IpAddress(127, 0, 0, 1);

        assert_eq!(street.get_address(), String::from("Main Street 46"));
        assert_eq!(postal_code.get_address(), String::from("13022"));
        assert_eq!(place.get_address(), String::from("Alexander Platz"));
        assert_eq!(ip_address.get_address(), String::from("127.0.0.1"));
    }

    
    #[test]
    fn option_test() {
        let required = 5;
        let optional = Some(5);
        let none = None;
        let sum = required + optional.unwrap_or(0) + none.unwrap_or(0);

        assert_eq!(sum, 10);
    }

    
    #[test]
    fn match_test() {
        let coin = Coin::Nickel;
        assert_eq!(coin.value_in_cents(), 5);

        let coin = Coin::Quarter(GermanyState::Berlin);
        assert_eq!(coin.value_in_cents(), 24);

        let coin = Coin::Quarter(GermanyState::Hamburg);
        assert_eq!(coin.value_in_cents(), 25);
    }

    
    #[test]
    fn match_with_option_test() {
        let five = Some(5);
        let zero = plus_one(Some(0));
        let six = plus_one(five);
        let none = plus_one(None);

        assert_eq!(six, Some(6));
        assert_eq!(none, None);
        assert_eq!(zero, None);
    }
}
