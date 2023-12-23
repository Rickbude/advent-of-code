use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    Button,
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<String, bool> },
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Parse the input. Memory modules are still uninitialized
    let mut tmp_modules: HashMap<String, Module> = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split(" -> ");
        let mut name = parts.next()?;
        let outputs: Vec<String> = parts
            .next()?
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();

        let mut module_type: ModuleType = ModuleType::Button;
        if name.chars().nth(0)? == '&' {
            name = name.strip_prefix('&')?;
            module_type = ModuleType::Conjunction {
                inputs: HashMap::new(),
            };
        } else if name.chars().nth(0)? == '%' {
            name = name.strip_prefix('%')?;
            module_type = ModuleType::FlipFlop { state: false };
        } else if name == "broadcaster" {
            module_type = ModuleType::Broadcast;
        }

        tmp_modules.insert(
            name.to_string(),
            Module {
                module_type: module_type,
                outputs: outputs,
            },
        );
    }

    let mut modules = tmp_modules.clone();
    //Set up input connections for conjunction modules
    for (target_name, target_module) in &mut modules {
        if let ModuleType::Conjunction { inputs } = &mut target_module.module_type {
            *inputs = HashMap::from_iter(tmp_modules.iter().filter_map(|(k, v)| {
                if v.outputs.contains(&target_name) {
                    Some((k.clone(), false))
                } else {
                    None
                }
            }));
        }
    }

    //For part 2, the solution is a bit ugly. Analysis of the input file reveals
    //that, in this case, rx is activated by
    //  &nd  -| => &hf => rx
    //  &pc  -|
    //  &tx  -|
    //  &vd  -|
    //By monitoring the activation cycles of the inputs of hf, we might figure
    //out the activation cycle of rx

    //Locate the conjunction node that outputs to rx (in this case hf)
    let modules_clone = modules.clone();
    let rx_exit_module = modules_clone
        .iter()
        .find(|x| x.1.outputs.contains(&"rx".to_string()));
    let mut rx_exit_cycles: HashMap<String, usize> = HashMap::new();
    if rx_exit_module.is_some() {
        if let ModuleType::Conjunction { inputs } = &rx_exit_module?.1.module_type {
            rx_exit_cycles = inputs.keys().clone().map(|v| (v.clone(), 0)).collect();
        }
    }

    let mut low_pulses_sent = 0;
    let mut high_pulses_sent = 0;

    let num_iterations = if part == 1 { 1000 } else { 10000 };
    for iteration in 0..num_iterations {
        let mut active_signals: VecDeque<(String, String, bool)> = VecDeque::new();
        active_signals.push_back(("button".to_string(), "broadcaster".to_string(), false));
        low_pulses_sent += 1;
        loop {
            //All signals were processed
            if active_signals.len() == 0 {
                break;
            }
            let (signal_from, signal_to, strength) = active_signals.pop_front()?;

            //Stop this pulse if it was sent to a non-existing module
            let module = match modules.get_mut(&signal_to) {
                None => continue,
                Some(module) => module,
            };

            //Process the input signal, determine output signal
            let output_signal = match &mut module.module_type {
                ModuleType::FlipFlop { state } => {
                    if strength {
                        None
                    } else {
                        *state = !*state;
                        Some(*state)
                    }
                }
                ModuleType::Broadcast => Some(strength),
                ModuleType::Conjunction { inputs } => {
                    inputs.insert(signal_from.clone(), strength);
                    Some(!inputs.values().all(|v| *v == true))
                }
                _ => None,
            };
            if output_signal.is_some() {
                for output in &module.outputs {
                    //Add new pulses to the que
                    active_signals.push_back((signal_to.clone(), output.clone(), output_signal?));

                    //See if the inputs to hf has turned on, store the current iteration index
                    if rx_exit_module.is_some()
                        && signal_to.clone() == rx_exit_module?.0.clone()
                        && strength == true
                    {
                        *rx_exit_cycles.get_mut(&signal_from)? = iteration + 1;
                        if !rx_exit_cycles.values().any(|v| v == &0) {
                            //Key part 2. For my input a regular product works.
                            //I don't know if this is just luck, or if it will
                            //work for all inputs. Other inputs might need LCM.
                            return Some(rx_exit_cycles.values().product());
                        }
                    }
                }
                if output_signal? {
                    high_pulses_sent += module.outputs.len();
                } else {
                    low_pulses_sent += module.outputs.len();
                }
            }
        }
    }
    //Key part 1
    Some(low_pulses_sent * high_pulses_sent)
}

fn main() -> std::io::Result<()> {
    //Calculate the answers to part 1 and 2
    println!("Key part 1: {}", calculate_key(1, "src/input.txt").unwrap());
    println!("Key part 2: {}", calculate_key(2, "src/input.txt").unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex1() {
        assert_eq!(calculate_key(1, "src/test_1.txt").unwrap(), 32000000);
    }

    #[test]
    fn test_pt1_ex2() {
        assert_eq!(calculate_key(1, "src/test_2.txt").unwrap(), 11687500);
    }
}
