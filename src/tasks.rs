
pub fn hello_world() {
    println!("Hello, World!");
}

pub fn fizz_buzz(n: u32) {
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

pub struct InventoryItem {
    pub name: String,
    pub count: u32,
}

pub fn is_available(inventory: &[InventoryItem], item_name: &str) {
    for item in inventory {
        if item.name == item_name {
            println!("{} is available ({})", item.name, item.count);
            return;
        }
    }
    println!("{} is not in inventory.", item_name);
}

pub fn is_palindrome(word: &str) -> bool {
    let new: String = word.chars().rev().collect();
    word == new
}

pub fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn c_to_k(c: f64) -> f64 {
    c + 273.15
}

pub fn k_to_c(k: f64) -> f64 {
    k - 273.15
}
