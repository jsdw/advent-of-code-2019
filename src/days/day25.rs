use crate::error::Error;
use crate::support::{ Terminal, IntoReadIter };
use std::io::{ stdin, stdout, Read };

// If interactive mode enabled, explore, pick things up, and find your
// way to the security door figuring out what items you need to hold to
// make you the correct weight. Else, run the script which works on
// my input specifically to give the answer that I needed.
pub fn part1(input: &str, interactive: bool) -> Result<(), Error> {

    // Read bytes from a script or interactively, depending on the flag
    let reader: Box<dyn Read> = if interactive {
        Box::new(stdin())
    } else {
        let mut cmds: Vec<u8> = vec![];
        for line in SCRIPT.trim().lines() {
            cmds.extend_from_slice(line.trim().as_bytes());
            cmds.push(b'\n');
        }
        Box::new(cmds.into_iter().into_reader())
    };

    // Pass in our reade, and output to stdout until the program ends.
    let mut terminal = Terminal::from_str(input, reader, stdout())?;
    while let Some(_) = terminal.step()? {}
    Ok(())
}


static SCRIPT: &str = "
    north
    west
    take antenna
    south
    take hologram
    west
    take astronaut ice cream
    east
    north
    north
    north
    north
    take space heater
    north
    east
    east
";