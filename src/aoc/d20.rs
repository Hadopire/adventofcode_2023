use std::collections::{HashMap, VecDeque};

enum ModuleKind {
    Out,
    FlipFlop,
    Conjunction,
    Broadcast,
}

struct Module {
    kind: ModuleKind,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    state: bool,
}

impl Module {
    fn new() -> Module {
        return Module { kind: ModuleKind::Out, inputs: Vec::new(), outputs: Vec::new(), state: false };
    }
}

pub fn solution(input: &str) -> (String, String) {
    let mut indices: HashMap<&str, usize> = HashMap::new();
    let mut modules: Vec<Module> = Vec::new();
    let mut rx_input = 0;

    for line in input.lines() {
        let (in_str, outs_str) = line.split_once(" -> ").unwrap();
        let module_kind = match in_str.as_bytes()[0] {
            b'%' => ModuleKind::FlipFlop,
            b'&' => ModuleKind::Conjunction,
            _ => ModuleKind::Broadcast,
        };

        let module_name = match module_kind {
            ModuleKind::Broadcast => in_str,
            _ => &in_str[1..],
        };

        let in_idx = if let Some(&idx) = indices.get(module_name) {
            idx
        } else {
            indices.insert(module_name, modules.len());
            modules.push(Module::new());
            modules.len() - 1
        };

        modules[in_idx].kind = module_kind;
        for out_str in outs_str.split(", ") {
            let out_idx = if let Some(&idx) = indices.get(out_str) {
                idx
            } else {
                indices.insert(out_str, modules.len());
                modules.push(Module::new());
                modules.len() - 1
            };

            if out_str == "rx" {
                rx_input = in_idx;
            }

            modules[in_idx].outputs.push(out_idx);
            modules[out_idx].inputs.push(in_idx);
        }
    }

    let mut queue: VecDeque<(usize, bool)> = VecDeque::new();
    let mut low = 0;
    let mut high = 0;
    let mut part_1 = 0;
    let mut part_2 = modules[rx_input].inputs.len() as u64;
    let mut rx_input_cycles = vec![0u64; modules[rx_input].inputs.len()];
    for cycle in 1..usize::MAX {
        low += 1;

        queue.push_back((indices["broadcaster"], false));
        while !queue.is_empty() {
            let (idx, pulse) = queue.pop_front().unwrap();
            let (state, skip) = match modules[idx].kind {
                ModuleKind::FlipFlop => (!modules[idx].state, pulse),
                ModuleKind::Broadcast => (pulse, false),
                ModuleKind::Conjunction => (!modules[idx].inputs.iter().all(|&i| modules[i].state), false),
                _ => (pulse, false),
            };

            if skip {
                continue;
            }

            if state {
                high += modules[idx].outputs.len();
            } else {
                low += modules[idx].outputs.len();
            }

            if idx == rx_input {
                for (i, &input) in modules[idx].inputs.iter().enumerate() {
                    if modules[input].state && rx_input_cycles[i] == 0 {
                        rx_input_cycles[i] = cycle as u64;
                        part_2 -= 1;
                    }
                }
            }

            modules[idx].state = state;
            for &out in modules[idx].outputs.iter() {
                queue.push_back((out, state));
            }
        }

        if cycle == 1000 {
            part_1 = low * high;
        }

        if part_2 == 0 {
            part_2 = rx_input_cycles.iter().product();
            break;
        }
    }

    return (part_1.to_string(), part_2.to_string());
}
