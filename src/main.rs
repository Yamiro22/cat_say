// src/main.rs
use cat_say_lib::{Options, process_options, process_input, print_cat};
use anyhow::Result;

fn main() -> Result<()> {
    let options = process_options()?;
    let message = process_input(&options)?;
    let eye = if options.dead { "x" } else { "o" };

    print_cat(&options, &message, eye)?;

    Ok(())
}
