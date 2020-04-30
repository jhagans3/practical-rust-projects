// extern crate structopt;

use structopt::StructOpt;
// cargo run -- --help
#[derive(StructOpt, Debug)]
struct Options {

    /// What does the cat say?
    #[structopt(short, long, default_value = "Meow!")]
    message: String,

    /// Make the cat appear dead
    #[structopt(short = "d", long = "dead")]
    dead: bool,
}


fn main() {
    let options = Options::from_args();
    let message = options.message;
    let status = if options.dead {
        "x"
    } else {
        "o"
    };

    // cargo run -- -m "woof" 1> stdout.txt 2> stderr.txt
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    println!("{}", message);
    println!("{pad}\\", pad=" ".repeat(1));
    println!("{pad}\\", pad=" ".repeat(2));
    println!("{pad}/\\_/\\", pad=" ".repeat(5));
    println!("{pad}( {eye} {eye} )", pad=" ".repeat(4), eye=status);
    println!("{pad}=( I )=", pad=" ".repeat(4));
}
