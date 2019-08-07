fn main() {
    let s1 = "パトカー";
    let s2 = "タクシー";
    let mut string = String::new();
    for i in 0..4 {
        string += &s1.chars().nth(i).unwrap().to_string();
        string += &s2.chars().nth(i).unwrap().to_string();
    }
    println!("{}", string);
}