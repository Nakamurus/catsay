extern crate colored;
extern crate structopt;

use std::io::{self, Read};
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "にゃお!")]
    /// what does the cat say?
    message: String,

    #[structopt(short = "d", long="dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    // Option<T>で包むことで、このフィールドがオプションだと示す
    // フィールドが提供されなければ、単にOption::Noneを返す
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool
}

// Box<dyn std::error::Error>は、std::error::Errorトレイトを
// 実装するあらゆるタイプを意味するトレイトオブジェクト
fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    // let message = std::env::args().nth(1)
        // .expect("Missing the message. Usage: catsay  < message>");
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message.bright_yellow().underline().on_purple());
    match &options.catfile {
        Some (path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        },
        None => {
            println!(" \\");
            println!("  \\");
            println!("    /\\_/\\");
            println!("   ( {eye} {eye} )", eye=eye.red().bold());
            println!("   =( I )=");
        }
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog");
    }
    Ok(())
}
