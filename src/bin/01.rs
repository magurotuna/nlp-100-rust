fn main() {
    let s = "パタトクカシーー";
    println!("{}", s.chars().step_by(2).collect::<String>());
}