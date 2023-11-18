use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};
use std::io::{self, Read};

#[derive(Parser)]
pub struct Options {
    #[clap(short = 'i', long = "stdin")]
    pub stdin: bool,
    #[clap(short = 'f', long = "file")]
    pub catfile: Option<std::path::PathBuf>,
    #[clap(default_value = "Meow!")]
    pub message: String,
    #[clap(short = 'd', long = "dead")]
    pub dead: bool,
}

pub fn process_options() -> Result<Options> {
    Ok(Options::parse())
}

pub fn process_input(options: &Options) -> Result<String> {
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message.clone();
    }

    if message.to_lowercase() == "woof" {
        return Err(anyhow::anyhow!("A cat shouldn't bark like a dog."));
    }

    Ok(message)
}

pub fn print_cat(options: &Options, message: &str, eye: &str) -> Result<()> {
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
        }
        None => {
            // Print the cat as before
            println!("{}", message.bright_blue().underline().on_bright_magenta());
            println!("\\");
            println!(" \\");
            println!("  /\\_/\\");
            println!(" ( {eye} {eye}", eye = eye.red().bold());
            println!(" =( I )=");
        }
    }

    Ok(())
}
