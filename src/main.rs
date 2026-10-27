fn main() {
    println!("Hello, world!");
}

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
/// play / leaderboard / quit
///     play : ask username
///         verify username
///         loop if not valid
///     leaderboard : print the top 3 scores
///     quit : say goodbye -> stop the program
///     wrong input : say it and ask again, never crash
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
///     back to the menu
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
