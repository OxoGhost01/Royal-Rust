use rand::Rng;

use crate::models::Unit;

// What happened during one wave.
pub struct FightReport {
    pub victory: bool,    // true = the monster died, false = the whole team died
    pub log: Vec<String>, // one text line per event, to print afterwards
}

// A wave : 4 steps. 1: heroes attacks monster ; 2: Check if monster still alive ; 3: Monster attacks 1 random alive hero ; 4: Check if one hero is still alive
pub fn fight_wave(team: &mut Vec<Unit>, monster: &mut Unit, rng: &mut impl Rng) -> FightReport {
    let mut log: Vec<String> = Vec::new();
    let mut round = 1;

    loop {
        log.push(format!("-- round {round} --"));

        for hero in team.iter_mut() {
            if !hero.is_alive() {
                continue;
            }
            let damage = hero.damage_against(monster);
            monster.take_damage(damage);
            log.push(format!("{} hits {} for {} (monster PV: {})", hero.name, monster.name, damage, monster.hp));

            if !monster.is_alive() {
                log.push(format!("{} is dead :'(", monster.name));
                return FightReport { victory: true, log };
            }
        }

        let mut alive_positions: Vec<usize> = Vec::new();
        for (position, hero) in team.iter().enumerate() {
            if hero.is_alive() {
                alive_positions.push(position);
            }
        }
        let pick = rng.random_range(0..alive_positions.len());
        let target = &mut team[alive_positions[pick]];

        let damage = monster.damage_against(target);
        target.take_damage(damage);
        log.push(format!("{} hits {} for {} (hero PV: {})", monster.name, target.name, damage, target.hp));
        if !target.is_alive() {
            log.push(format!("{} is dead...", target.name));
        }

        let mut someone_alive = false;
        for hero in team.iter() {
            if hero.is_alive() {
                someone_alive = true;
            }
        }
        if !someone_alive {
            return FightReport { victory: false, log };
        }

        round += 1;
    }
}
