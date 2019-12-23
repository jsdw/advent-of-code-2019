use crate::error::Error;
use crate::support::{ Terminal, Uppercase };
use std::io::{ stdin, stdout };

pub fn both_parts(input: &str) -> Result<(), Error> {
    let mut terminal = Terminal::from_str(input, Uppercase::new(stdin()), stdout())?;

    if let Some(n) = terminal.step()? {
        print!("Made it across! Hull damage: {}", n);
    }

    Ok(())
}
