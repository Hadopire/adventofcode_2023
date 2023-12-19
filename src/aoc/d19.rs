use std::{collections::HashMap, ops::Range};

struct Rule<'a> {
    category: Option<usize>,
    operator: Option<u8>,
    operand: Option<u64>,
    next_workflow: &'a str,
}

fn category_idx(category: u8) -> usize {
    return match category {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => 0,
    };
}

fn all_combinations(mut ranges: [Range<u64>; 4], workflow: &[Rule], workflows: &HashMap<&str, Vec<Rule>>) -> u64 {
    let mut count = 0;

    for rule in workflow {
        let mut inversed_range = 0..0;
        if let (Some(category), Some(operator), Some(operand)) = (rule.category, rule.operator, rule.operand) {
            let r = &ranges[category];
            (ranges[category], inversed_range) = match operator {
                b'>' => ((operand + 1)..r.end, r.start..(operand + 1)),
                b'<' => (r.start..operand, operand..r.end),
                _ => panic!(),
            }
        }

        match rule.next_workflow {
            "A" => count += ranges.iter().fold(1, |acc, range| acc * (range.end - range.start)),
            "R" => (),
            _ => count += all_combinations(ranges.clone(), &workflows[rule.next_workflow], workflows),
        }

        if let Some(category) = rule.category {
            ranges[category] = inversed_range;
        }
    }

    return count;
}

pub fn solution(input: &str) -> (String, String) {
    let (workflows_input, parts_input) = input.split_once("\n\n").unwrap();
    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::new();

    for line in workflows_input.lines() {
        let (name, rules_str) = line.split_once('{').unwrap();
        workflows.insert(
            name,
            rules_str
                .trim_end_matches('}')
                .split(',')
                .map(|s| {
                    let opt = s.split_once(':');
                    if let Some((op, result)) = opt {
                        Rule {
                            category: Some(category_idx(op.as_bytes()[0])),
                            operator: Some(op.as_bytes()[1]),
                            operand: Some(op[2..].parse().unwrap()),
                            next_workflow: result,
                        }
                    } else {
                        Rule { category: None, operator: None, operand: None, next_workflow: s }
                    }
                })
                .collect(),
        );
    }

    let mut part_1 = 0;
    for line in parts_input.lines() {
        let mut part = [0u64; 4];
        for category in line[1..line.len() - 1].split(',') {
            part[category_idx(category.as_bytes()[0])] = category[2..].parse().unwrap();
        }

        let mut workflow = &workflows["in"];
        let mut accepted: Option<bool> = None;
        while accepted == None {
            for rule in workflow.iter() {
                if let (Some(category), Some(operator), Some(operand)) = (rule.category, rule.operator, rule.operand) {
                    let pass = match operator {
                        b'>' => part[category] > operand,
                        b'<' => part[category] < operand,
                        _ => panic!(),
                    };

                    if !pass {
                        continue;
                    }
                }

                match rule.next_workflow {
                    "A" => accepted = Some(true),
                    "R" => accepted = Some(false),
                    _ => workflow = &workflows[rule.next_workflow],
                }
                break;
            }
        }

        if accepted.unwrap() {
            part_1 += part.iter().sum::<u64>();
        }
    }

    let part_2 = all_combinations([1..4001, 1..4001, 1..4001, 1..4001], &workflows["in"], &workflows);

    return (part_1.to_string(), part_2.to_string());
}
