use std::collections::HashMap;

fn main() {
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let mut split = s.split_whitespace();
    let special_word_index: [usize;9] = [0, 4, 5, 6, 7, 8, 14, 15, 18];

    let mut answer = HashMap::new();
    let mut index = 0;
    while let Some(word) = split.next() {
        let key;
        if special_word_index.iter().any(|x| *x == index) {
            key = word.chars().nth(0).unwrap().to_string();
        } else {
            key = word.chars().take(2).collect::<String>();
        }
        index += 1;
        answer.insert(key, index);
    }

    println!("{:?}", answer);
}