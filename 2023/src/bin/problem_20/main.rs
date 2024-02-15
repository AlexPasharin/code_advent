#![warn(clippy::all, clippy::pedantic)]

use num::integer::lcm;

use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, VecDeque},
};

// use regex::Regex;
use utils::file_reader::FileReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseType {
    HighPulse,
    LowPulse,
}

use PulseType::*;

#[derive(Debug)]
enum Module {
    Broadcaster,
    FlipFlopModule(FlipFlopModule),
    ConjuctionModule(ConjuctionModule),
}

use Module::*;

#[derive(Debug)]
struct FlipFlopModule {
    on: Cell<bool>,
}

#[derive(Debug)]
struct ConjuctionModule {
    input_module_last_pulses: RefCell<HashMap<String, PulseType>>,
}

#[derive(Debug)]
struct ModuleData {
    module: Module,
    destination_modules: Vec<String>,
}

struct PulseSent<'a> {
    input_module: &'a str,
    destination_module: &'a str,
    pulse: PulseType,
}

struct PulseQueue<'a> {
    queue: VecDeque<PulseSent<'a>>,
    high_pulses_sent: u32,
    low_pulses_sent: u32,
}

impl<'a> PulseQueue<'a> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            high_pulses_sent: 0,
            low_pulses_sent: 0,
        }
    }

    fn push_back(&mut self, pulse_sent: PulseSent<'a>, update_score: bool) {
        if update_score {
            match &pulse_sent.pulse {
                HighPulse => self.high_pulses_sent += 1,
                LowPulse => self.low_pulses_sent += 1,
            }
        }

        self.queue.push_back(pulse_sent);
    }

    fn pop_front(&mut self) -> Option<PulseSent<'a>> {
        self.queue.pop_front()
    }
}

static BROADCASTER: &str = "broadcaster";

fn main() {
    let mut modules = HashMap::new();

    FileReader::process_lines("./input/problem20.txt", &mut |line| {
        let (name, destination_modules) = line.split_once(" -> ").unwrap();

        let (module, name) = if name == BROADCASTER {
            (Broadcaster, name.to_owned())
        } else {
            let mut chars = name.chars();
            let first_ch = chars.next().unwrap();
            let name = chars.collect::<String>();

            let module = match first_ch {
                '%' => FlipFlopModule(FlipFlopModule {
                    on: Cell::new(false),
                }),
                '&' => ConjuctionModule(ConjuctionModule {
                    input_module_last_pulses: RefCell::new(HashMap::new()),
                }),
                ch => unreachable!("Name of module cannot start with the character {}", ch),
            };

            (module, name)
        };

        modules.insert(
            name,
            ModuleData {
                module,
                destination_modules: destination_modules.split(", ").map(String::from).collect(),
            },
        );
    });

    for (module_name, module) in &modules {
        for destination_module_name in &module.destination_modules {
            if let Some(ConjuctionModule(conjuction_module)) =
                &modules.get(destination_module_name).map(|m| &m.module)
            {
                let mut input_module_last_pulses =
                    conjuction_module.input_module_last_pulses.borrow_mut();

                input_module_last_pulses.insert(module_name.to_string(), LowPulse);
            }
        }
    }

    // solution for part 2 involves some "cheating" - a lot of assumptions that are not checked and manual hardcoded values for a particular input
    // "rx" has only one input module - "cl", which is a conjuction module
    // there are 4 input modules to cl, "js", "qs", "dt" and "ts" which are all also conjuction modules
    // if we make a lot of assumptions about the graph and the state going in cycles regularly,
    // we see that the solution is lcm of amount of rounds it takes for each of this 4 modules to recieve a low pulse for the first time

    let mut final_conjuctions = [("js", None), ("qs", None), ("dt", None), ("ts", None)];

    let mut pulse_queue = PulseQueue::new();

    for round in 1.. {
        let update_score = round <= 1000;

        if round >= 1000 && final_conjuctions.iter().all(|(_, val)| val.is_some()) {
            break;
        }

        pulse_queue.push_back(
            PulseSent {
                input_module: BROADCASTER,
                destination_module: BROADCASTER,
                pulse: LowPulse,
            },
            update_score,
        );

        while let Some(PulseSent {
            input_module,
            pulse,
            destination_module,
        }) = pulse_queue.pop_front()
        {
            if pulse == LowPulse {
                let final_conjuction = final_conjuctions
                    .iter_mut()
                    .find(|(name, _)| *name == destination_module);

                if let Some(final_conjuction) = final_conjuction {
                    if final_conjuction.1.is_none() {
                        final_conjuction.1 = Some(round);
                    }
                }
            }

            if let Some(ModuleData {
                module,
                destination_modules,
            }) = modules.get(destination_module)
            {
                let next_pulse = match module {
                    FlipFlopModule(flipflop_module) => match pulse {
                        LowPulse => {
                            let module_is_on = flipflop_module.on.get();

                            let next_pulse = if module_is_on { LowPulse } else { HighPulse };

                            flipflop_module.on.set(!module_is_on);

                            Some(next_pulse)
                        }
                        HighPulse => None,
                    },
                    ConjuctionModule(conjuction_module) => {
                        let mut input_module_last_pulses =
                            conjuction_module.input_module_last_pulses.borrow_mut();

                        // pulse_type = input_module_last_pulses.get_mut(module_name).unwrap();
                        input_module_last_pulses.insert(input_module.to_owned(), pulse);

                        let all_last_input_pulses_are_high = input_module_last_pulses
                            .values()
                            .all(|pulse| pulse == &HighPulse);

                        Some(if all_last_input_pulses_are_high {
                            LowPulse
                        } else {
                            HighPulse
                        })
                    }
                    Broadcaster => Some(LowPulse),
                };

                if let Some(next_pulse) = next_pulse {
                    destination_modules.iter().for_each(|module_name| {
                        pulse_queue.push_back(
                            PulseSent {
                                input_module: destination_module,
                                destination_module: module_name,
                                pulse: next_pulse,
                            },
                            update_score,
                        );
                    });
                };
            }
        }
    }

    let PulseQueue {
        high_pulses_sent,
        low_pulses_sent,
        ..
    } = pulse_queue;

    println!("High pulses sent: {}", high_pulses_sent);
    println!("Low pulses sent: {}", low_pulses_sent);
    println!("Multiplied: {}", high_pulses_sent * low_pulses_sent); // 938065580

    let rounds = final_conjuctions.map(|(_, x)| x.unwrap());
    println!("Low pulse sent to rx after {} rounds", gen_lcm(rounds)); // 250628960065793
}
fn gen_lcm(numbers: [i64; 4]) -> i64 {
    numbers.into_iter().reduce(|acc, n| lcm(acc, n)).unwrap()
}
