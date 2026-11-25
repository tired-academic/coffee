fn main() {
    let args: Vec<String> = std::env::args().collect();
    let prog: &String = &args[0];
    println!("Prog: {prog}");
}