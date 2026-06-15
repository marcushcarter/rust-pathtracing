

pub fn reverse_string(word: &str) {
    // let num = word.chars().;
    // println!("{}", num);

    let new: String = word.chars().rev().collect();
    println!("{}", new);
}