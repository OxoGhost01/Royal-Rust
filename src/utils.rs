use std::io::{self, Write};

use crate::models::{ScoreEntry, Unit};

pub enum Flow {
    Continue, // go back to the menu
    Quit,     // leave the program
}

// `C` = the type of the "context" given to every action (the database, the heroes...).
pub struct MenuEntry<C> {
    /// Text shown to the player.
    /// `&'static str` = a text literal written in the source code, which lives for the whole program (so the struct does not need to own/free it).
    pub label: &'static str,
    /// The action. (okay i shut up you knew it)
    pub action: fn(&C) -> Flow,
}

pub fn choose_menu<'a, C>(title: &str, entries: &'a [MenuEntry<C>]) -> io::Result<&'a MenuEntry<C>> {
    println!("\n--- {title} ---");

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

pub fn ask_username() -> io::Result<String> {
    loop {
        let name = read_line("Your name: ")?;
        if name.is_empty() {
            println!("The name cannot be empty.");
        }
        else if name.chars().count() < 3 || name.chars().count() > 20 {
            println!("The name must be between 3 and 20 caracters :/")
        } 
        else {
            return Ok(name);
        }
    }
}

pub fn pick_team(roster: &[Unit], size: usize) -> io::Result<Vec<Unit>> {
    let mut available: Vec<Unit> = roster.to_vec();
    let mut team: Vec<Unit> = Vec::new();

    while team.len() < size {
        println!("\nPick hero {} of {}:", team.len() + 1, size);
        for (i, hero) in available.iter().enumerate() {
            println!("  {:>2}. {}", i + 1, hero);
        }
        let choice = read_number("Your hero: ", 1, available.len())?;
        let hero = available.remove(choice - 1);
        println!("{} joins your team", hero.name);
        team.push(hero);
    }
    Ok(team)
}

pub fn print_team(team: &[Unit]) {
    println!("Your team:");
    for hero in team {
        if hero.is_alive() {
            println!("  {}", hero);
        } else {
            println!("  {} (dead)", hero.name);
        }
    }
}

pub fn print_leaderboard(scores: &[ScoreEntry]) {
    println!("\n--- Top scores ---");
    if scores.is_empty() {
        println!("No score yet.");
    }
    for (i, score) in scores.iter().enumerate() {
        println!("  {}. {:<20} {} waves", i + 1, score.player_name, score.waves);
    }
}

pub fn print_wave_info(wave_number: u32, monster: &Unit, team: &[Unit]) {
    println!("\n-------- WAVE {wave_number} --------");
    println!("A wild monster appears ! --- It is {monster}");
    print_team(team);
    println!();
}

pub fn print_log(log: &[String]) {
    for line in log {
        println!("{line}");
    }
}

pub fn print_game_over(username: &str, waves: u32) {
    println!("\nGAME OVER {username} ! You survived {waves} waves. (looser, HA)");
}
