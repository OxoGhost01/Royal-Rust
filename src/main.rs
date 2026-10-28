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
///     print the menu table as a numbered list
///     ask a number, wrong input : say it and ask again, never crash
///     run the action of the chosen line (see actions)
///     the action answers what to do next (Flow) :
///         Continue : go back to the start of the loop, show the menu again
///         Quit : leave the loop -> say goodbye -> stop the program
/// actions :
///     play : ask username
///         verify username
///         loop if not valid
///         heroes choice (see Heroes choice)
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

mod db_init;
mod game;
mod models;
mod utils;

use db_init::Repo;
use models::{ScoreEntry, Unit};
use utils::{Flow, MenuEntry, choose_menu};
use std::{thread::sleep, time::Duration};

// Number of heroes in a team
const TEAM_SIZE: usize = 3;

struct Context {
    rt: tokio::runtime::Runtime,
    repo: Repo,
    heroes: Vec<Unit>,
    monsters: Vec<Unit>,
}

// MENU ACTIONS
// Each action receives the context and returns a `Flow`.
fn play(ctx: &Context) -> Flow {
    match play_game(ctx) {
        Ok(()) => Flow::Continue,
        Err(e) => {
            println!("Input error: {e}");
            Flow::Quit
        }
    }
}

fn play_game(ctx: &Context) -> std::io::Result<()> {
    let username = utils::ask_username()?;
    let mut team = utils::pick_team(&ctx.heroes, TEAM_SIZE)?;

    let mut rng = rand::rng();
    let mut waves = 0;
    loop {
        let index = rand::Rng::random_range(&mut rng, 0..ctx.monsters.len());
        let mut monster = ctx.monsters[index].clone();

        println!("\n-------- WAVE {} --------", waves + 1);
        println!("A wild monster appears ! --- It is {monster}");
        utils::print_team(&team);
        println!();

        let report = game::fight_wave(&mut team, &mut monster, &mut rng);
        for line in &report.log {
            println!("{line}");
        }

        if report.victory {
            waves += 1;
            println!("Wave won ! ({waves} waves won)");
        } else {
            break;
        }
        sleep(Duration::from_millis(1000));
    }

    println!("\nGAME OVER {username} ! You survived {waves} waves. (looser, HA)");
    let entry = ScoreEntry { player_name: username, waves };
    if let Err(e) = ctx.rt.block_on(ctx.repo.save_score(&entry)) {
        println!("Could not save your score: {e}");
    }
    leaderboard(ctx);
    Ok(())
}

fn leaderboard(ctx: &Context) -> Flow {
    match ctx.rt.block_on(ctx.repo.top_scores(3)) {
        Ok(scores) => utils::print_leaderboard(&scores),
        Err(e) => println!("Could not read the scores: {e}"),
    }
    Flow::Continue
}

fn quit(_ctx: &Context) -> Flow {
    println!("so sad you already quit the game ?");
    Flow::Quit
}

const MENU: &[MenuEntry<Context>] = &[
    MenuEntry { label: "Play", action: play },
    MenuEntry { label: "Leaderboard", action: leaderboard },
    MenuEntry { label: "Quit", action: quit },
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let mongo_uri = std::env::var("MONGODB_URI")?;
    let mongo_db = std::env::var("MONGODB_DB")?;
    println!("Config loaded (database: {mongo_db})");

    // Each `?` stops the program with the error message if that step fails.
    let rt = tokio::runtime::Runtime::new()?;
    let repo = rt.block_on(Repo::connect(&mongo_uri, &mongo_db))?;
    let heroes = rt.block_on(repo.all_characters())?;
    let monsters = rt.block_on(repo.all_monsters())?;
    println!("Loaded {} heroes and {} monsters", heroes.len(), monsters.len());

    if heroes.len() < TEAM_SIZE || monsters.is_empty() {
        return Err("not enough heroes or monsters in the database".into());
    }

    let ctx = Context { rt, repo, heroes, monsters };

    loop {
        let entry = choose_menu("Clash Not Royal", MENU)?;
        match (entry.action)(&ctx) {
            Flow::Continue => {}
            Flow::Quit => break,
        }
    }

    println!("Bye!");
    Ok(())
}
