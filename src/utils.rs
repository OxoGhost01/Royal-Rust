use std::io::{self, Write};

pub enum Flow {
    Continue, // go back to the menu
    Quit,     // leave the program
}

/// One line of the menu.
pub struct MenuEntry {
    /// Text shown to the player.
    /// `&'static str` = a text literal written in the source code, which lives for the whole program (so the struct does not need to own/free it).
    pub label: &'static str,
    /// The action. (okay i shut up you knew it)
    pub action: fn() -> Flow,
}

pub fn choose_menu<'a>(title: &str, entries: &'a [MenuEntry]) -> io::Result<&'a MenuEntry> {
    println!("\n=== {title} ===");

    for (i, entry) in entries.iter().enumerate() {
        println!("  {}. {}", i + 1, entry.label);
    }

    let n = read_number("Your choice: ", 1, entries.len())?;
    Ok(&entries[n - 1])
}

pub fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut buf = String::new();
    // read_line APPENDS to buf and returns how many bytes it read.
    let bytes = io::stdin().read_line(&mut buf)?;
    if bytes == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "input closed"));
    }
    Ok(buf.trim().to_string())
}

/// `usize` because it is the type used to index a list.
pub fn read_number(prompt: &str, min: usize, max: usize) -> io::Result<usize> {
    loop {
        let input = read_line(prompt)?;
        match input.parse::<usize>() {
            Ok(n) if min <= n && n <= max => return Ok(n),
            _ => println!("Please type a number between {min} and {max}."),
        }
    }
}
