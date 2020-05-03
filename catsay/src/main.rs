use colored::*;
use structopt::StructOpt;

// cargo run -- --help
#[derive(StructOpt, Debug)]
struct Options {
    /// What does the cat say?
    #[structopt(short, long, default_value = "Meow!")]
    message: String,

    /// Make the cat appear dead
    #[structopt(short, long)]
    dead: bool,

    /// Load the cat picture from the specified file
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    catfile: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    // NO_COLOR=1 cargo run
    let status = if options.dead {
        "x".red().bold()
    } else {
        "o".green().bold()
    };

    // cargo run -- -m "woof" 1> stdout.txt 2> stderr.txt
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    println!("{}", message.bright_yellow().underline().on_purple());
    match &options.catfile {
        Some(path) => {
            let cat_template =
                std::fs::read_to_string(path).expect(&format!("could not read file {:?}", path));
            let cat_picture = cat_template.replace("{eye}", "o");
            println!("{}", &cat_picture);
        }
        None => {
            println!("{pad}\\", pad = " ".repeat(1));
            println!("{pad}\\", pad = " ".repeat(2));
            println!("{pad}/\\_/\\", pad = " ".repeat(5));
            println!("{pad}( {eye} {eye} )", pad = " ".repeat(4), eye = status);
            println!("{pad}=( I )=", pad = " ".repeat(4));
        }
    }
}
