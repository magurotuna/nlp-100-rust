fn create_word_ngram(s: &str, n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let max = s.split_whitespace().count() - n + 1; // n-gramを生成できる個数

    for i in 0..max {
        let split = s.split_whitespace();
        let ngram_elem = split.skip(i).take(n)
            .fold(String::from(""), |acc, x| format!("{} {}", acc, x));
        result.push(ngram_elem);
    }
    result
}

fn create_char_ngram(s: &str, n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let max = s.len() - n + 1;
    for i in 0..max {
        result.push(s.get(i..(n + i)).unwrap().to_string());
    }
    result
}

fn main() {
    let s = "I am an NLPer";
    println!("word: {:?}", create_word_ngram(s, 2));
    println!("char: {:?}", create_char_ngram(s, 2));
}