/// # Small
/// connect to db and import monsters + heros
/// show menu
/// take player name and choice
///     print choice
///     get input
///     check input
/// play the game :
///     heroes choice :
///         print all available heroes
///         user choose (verify input)
///     turns :
///         print caracters life
///         clash with debug
///
/// # Full
///
/// # Menu
/// loop :
///     print the table as a numbered list
///     ask a number, wrong input : say it and ask again, never crash
///     run the action of the chosen line
///     the action answers what to do next (Flow) :
///         Continue : go back to the start of the loop, show the menu again
///         Quit : leave the loop -> say goodbye -> stop the program
/// actions :
///     play : ask username
///         verify username
///         loop if not valid
///         heroes choice
///         play the game, then answer Continue (back to the menu)
///     leaderboard : print the top 3 scores, then answer Continue
///     quit : answer Quit
///
/// # Heroes choice
/// pick 3 different heroes
///     a hero already picked is not shown anymore
///     wrong input : ask again
///
/// # Game
/// wave counter starts at 0
/// loop :
///     take a random monster
///     loop : fight until the monster or the whole team is dead
///         every living hero hits the monster
///         if the monster is dead : wave won, counter + 1, next wave
///         the monster hits one random living hero
///         if all heroes are dead : game over
///     apply effects
/// game over :
///     save username + number of waves
///     show the top 3
///     back to the menu (return Continue)
///
/// # Damage
/// attack - defense, but at least 1 (so a fight can always end)
///
/// # Files
/// models : hero / monster / score
/// db_init : everything that touches the database
/// game : the fight logic
/// utils : inputs and prints
/// main : menu and game loop

mod models;
mod utils;

use utils::{Flow, MenuEntry, choose_menu};

// MENU ACTIONS
// Each action is a function with no argument that returns a `Flow`.

fn play() -> Flow {
    println!("yeah the game is good, huh ?");
    Flow::Continue
}

fn leaderboard() -> Flow {
    println!("no leaderboard means you are the best on your own :)");
    Flow::Continue
}

fn quit() -> Flow {
    println!("so sad you already quit the game ?");
    Flow::Quit
}

const MENU: &[MenuEntry] = &[
    MenuEntry { label: "Play", action: play },
    MenuEntry { label: "Leaderboard", action: leaderboard },
    MenuEntry { label: "Quit", action: quit },
];

// `Box<dyn Error>` = "any kind of error"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // (the leading `_` tells the compiler "I know this is unused for now")
    let _mongo_uri = std::env::var("MONGODB_URI")?;
    let mongo_db = std::env::var("MONGODB_DB")?;
    println!("Config loaded (database: {mongo_db})");

    loop {
        let entry = choose_menu("Clash Not Royal", MENU)?;
        match (entry.action)() {
            Flow::Continue => {}
            Flow::Quit => break,
        }
    }

    println!("Bye!");
    Ok(())
}
