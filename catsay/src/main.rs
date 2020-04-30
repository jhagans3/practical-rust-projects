fn main() {
    let message = std::env::args()
        .nth(1) // the 0 arg is the name of the binary
        .expect("Missing the message. Usage: catsay < message>");
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    =( I )=");
}
