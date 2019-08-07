fn main() {
    let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let answer: Vec<usize> = s.replace(",", "").replace(".", "").split_whitespace()
        .map(|w| w.len()).collect();
    println!("{:?}", answer);
}