use std::collections::LinkedList;

#[derive(Copy, Clone, PartialEq, Eq)]
enum EffectType {
    Shield,
    Poison,
    Recharge,
}

struct Spell {
    cost: i32,
    damage: i32,
    heals: i32,
    effect: Option<Effect>,
}

#[derive(Clone)]
struct Effect {
    count: i32,
    effect_type: EffectType,
    index: usize,
}

struct State {
    player_hit_points: i32,
    boss_hit_points: i32,
    mana: i32,
    effects: Vec<Effect>,
    next_spell_idx: usize,
    transition_cost: i32,
}

const BOSS_DAMAGE: i32 = 9;
const BOSS_INITIAL_HIT_POINTS: i32 = 58;

const PLAYER_INITIAL_HIT_POINTS: i32 = 50;
const PLAYER_INITIAL_MANA: i32 = 500;

const MAGIC_MISSILE: Spell = Spell {
    cost: 53,
    damage: 4,
    heals: 0,
    effect: None,
};

const DRAIN: Spell = Spell {
    cost: 73,
    damage: 2,
    heals: 2,
    effect: None,
};

const SHIELD_EFFECT: Effect = Effect {
    count: 6,
    effect_type: EffectType::Shield,
    index: 2,
};

const SHIELD: Spell = Spell {
    cost: 113,
    damage: 0,
    heals: 0,
    effect: Some(SHIELD_EFFECT),
};

const POISON_EFFECT: Effect = Effect {
    count: 6,
    effect_type: EffectType::Poison,
    index: 3,
};

const POISON: Spell = Spell {
    cost: 173,
    damage: 0,
    heals: 0,
    effect: Some(POISON_EFFECT),
};

const RECHARGE_EFFECT: Effect = Effect {
    count: 5,
    effect_type: EffectType::Recharge,
    index: 4,
};

const RECHARGE: Spell = Spell {
    cost: 229,
    damage: 0,
    heals: 0,
    effect: Some(RECHARGE_EFFECT),
};

const SPELLS: [Spell; 5] = [MAGIC_MISSILE, DRAIN, SHIELD, POISON, RECHARGE];
const SPELLS_AMOUNT: usize = SPELLS.len();

struct Path {
    path: LinkedList<State>,
    cost: i32,
}

fn main() {
    let minimal_winning_cost_easy_mode = game_simulation(false);
    let minimal_winning_cost_difficult_mode = game_simulation(true);

    println!(
        "Minimal winning cost, part 1: {}",
        minimal_winning_cost_easy_mode
    ); // 1269

    println!(
        "Minimal winning cost, part 2: {}",
        minimal_winning_cost_difficult_mode
    ); // 1309
}

fn game_simulation(difficult_mode: bool) -> i32 {
    let mut minimal_winning_cost = i32::MAX;

    let initial_state = State {
        player_hit_points: PLAYER_INITIAL_HIT_POINTS,
        mana: PLAYER_INITIAL_MANA,
        boss_hit_points: BOSS_INITIAL_HIT_POINTS,
        effects: Vec::new(),
        next_spell_idx: 0,
        transition_cost: 0,
    };

    let mut path = LinkedList::new();
    path.push_front(initial_state);

    let mut current_path = Path { path, cost: 0 };

    loop {
        let latest_state = current_path.path.front_mut();

        match latest_state {
            None => break, // no more states
            Some(state) => {
                let mut next_player_hit_points = state.player_hit_points;

                if difficult_mode {
                    next_player_hit_points -= 1;

                    if next_player_hit_points <= 0 {
                        // player loses immediately in the difficult mode
                        backtrack(&mut current_path);
                        continue;
                    }
                }

                let next_spell_idx = state.next_spell_idx;

                if next_spell_idx >= SPELLS_AMOUNT {
                    // no more spells to cast from this state
                    backtrack(&mut current_path);
                    continue;
                }

                // if next_spell_idx correspond to an effect on current state (unless it is it's last round)
                if state
                    .effects
                    .iter()
                    .any(|f| f.count > 1 && f.index == next_spell_idx)
                {
                    state.next_spell_idx += 1; // try next spell in the next round
                    continue;
                }

                let spell = &SPELLS[next_spell_idx];

                // can't afford next spell
                if spell.cost > state.mana {
                    state.next_spell_idx += 1; // try next spell in the next round
                    continue;
                }

                // player turn -  calculate new cost of path
                let new_cost = current_path.cost + spell.cost;

                // if cost is already bigger than minimal_winning_cost, no sense in countuining this path
                if new_cost >= minimal_winning_cost {
                    state.next_spell_idx += 1; // try next spell in the next round
                    continue;
                }

                let mut next_boss_hit_points = state.boss_hit_points - spell.damage;
                next_player_hit_points += spell.heals;
                let mut next_mana = state.mana - spell.cost;

                // apply effects, at the same time form new effect by
                // adjusting effects count, through away expired ones
                let mut apply_effects = |effects: &Vec<Effect>| -> Vec<_> {
                    effects
                        .iter()
                        .filter_map(|f| {
                            match f.effect_type {
                                EffectType::Shield => {}
                                EffectType::Poison => {
                                    next_boss_hit_points -= 3;
                                }
                                EffectType::Recharge => next_mana += 101,
                            }

                            let new_count = f.count - 1;

                            if new_count > 0 {
                                Some(Effect {
                                    count: new_count,
                                    effect_type: f.effect_type,
                                    index: f.index,
                                })
                            } else {
                                None
                            }
                        })
                        .collect()
                };

                // apply effects and get new ones for player's turn
                let mut new_effects = apply_effects(&state.effects);

                // if cast spell has effect, add it to effects. it will start effecting starting from the next turn (boss' turn)
                if let Some(effect) = &spell.effect {
                    new_effects.push(effect.clone());
                }

                // boss turn

                // calculate if player has armor at the moment. this depends on current effects, so needs to be done before we mutate as a side effect of applying them later
                let player_has_armor = new_effects
                    .iter()
                    .any(|effect| effect.effect_type == EffectType::Shield);

                // apply effects and get new ones for boss's turn
                new_effects = apply_effects(&new_effects);

                // check if boss lost the game - either as a result of player's turn of applying new effects in the start of the boss turn
                if next_boss_hit_points <= 0 {
                    minimal_winning_cost = new_cost;

                    backtrack(&mut current_path);
                    continue;
                }

                let player_armor = if player_has_armor { 7 } else { 0 };
                let damage_by_boss = std::cmp::max(BOSS_DAMAGE - player_armor, 1);
                next_player_hit_points -= damage_by_boss;

                if next_player_hit_points <= 0 {
                    // player loses as a result of boss attack
                    backtrack(&mut current_path);
                    continue;
                }

                // add new spell to path as a new state and adjust path's cost
                let next_state = State {
                    player_hit_points: next_player_hit_points,
                    boss_hit_points: next_boss_hit_points,
                    mana: next_mana,
                    effects: new_effects,
                    next_spell_idx: 0,
                    transition_cost: spell.cost,
                };

                current_path.path.push_front(next_state);
                current_path.cost += spell.cost;
            }
        }
    }

    return minimal_winning_cost;
}

// remove current head, retract its transition cost from the path, and adjust new head's next_spell_idx
fn backtrack(path: &mut Path) {
    let head = path.path.pop_front();

    if let Some(new_head) = path.path.front_mut() {
        new_head.next_spell_idx += 1;
    }

    if let Some(old_head) = head {
        path.cost -= old_head.transition_cost;
    }
}
