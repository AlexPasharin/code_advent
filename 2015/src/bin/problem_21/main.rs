use std::cmp::{max, min};

struct Weapon {
    cost: i32,
    damage: i32,
}

struct Armor {
    cost: i32,
    armor: i32,
}

struct Ring {
    cost: i32,
    damage: i32,
    armor: i32,
}

fn main() {
    let weapons = [
        Weapon { cost: 8, damage: 4 },
        Weapon {
            cost: 10,
            damage: 5,
        },
        Weapon {
            cost: 25,
            damage: 6,
        },
        Weapon {
            cost: 40,
            damage: 7,
        },
        Weapon {
            cost: 74,
            damage: 8,
        },
    ];

    let armors = [
        Armor { cost: 0, armor: 0 },
        Armor { cost: 13, armor: 1 },
        Armor { cost: 31, armor: 2 },
        Armor { cost: 53, armor: 3 },
        Armor { cost: 75, armor: 4 },
        Armor {
            cost: 102,
            armor: 5,
        },
    ];

    let original_rings = [
        Ring {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Ring {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Ring {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Ring {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Ring {
            cost: 20,
            armor: 1,
            damage: 0,
        },
        Ring {
            cost: 40,
            armor: 2,
            damage: 0,
        },
        Ring {
            cost: 80,
            armor: 3,
            damage: 0,
        },
    ];

    let original_rings_amount = original_rings.len();

    let mut rings: Vec<Ring> = original_rings.into_iter().collect();

    for i in 1..original_rings_amount {
        for j in (i + 1)..original_rings_amount {
            let ring1 = &rings[i];
            let ring2 = &rings[j];

            let new_ring = Ring {
                cost: ring1.cost + ring2.cost,
                damage: ring1.damage + ring2.damage,
                armor: ring1.armor + ring2.armor,
            };

            rings.push(new_ring);
        }
    }

    let mut min_win_cost = i32::MAX;
    let mut max_lose_cost = i32::MIN;

    for weapon in &weapons {
        for armor in &armors {
            for ring in &rings {
                let damage_to_boss = weapon.damage + ring.damage - 2;
                let damage_to_player = max(8 - armor.armor - ring.armor, 1);
                let cost = weapon.cost + armor.cost + ring.cost;

                if (109.0 / damage_to_boss as f64).ceil()
                    <= (100.0 / damage_to_player as f64).ceil()
                {
                    // player wins
                    min_win_cost = min(min_win_cost, cost); // 111
                } else {
                    //player loses
                    max_lose_cost = max(max_lose_cost, cost); // 188
                }
            }
        }
    }

    println!("Minimal winning cost: {}", min_win_cost);
    println!("Maximal losing cost: {}", max_lose_cost);
}
