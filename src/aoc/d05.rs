use std::cmp::min;
use std::ops::Range;

fn intersect<T: Ord + Copy>(a: &Range<T>, b: &Range<T>) -> Option<Range<T>> {
    if a.start < b.end && b.start < a.end {
        return Some(std::cmp::max(a.start, b.start)..std::cmp::min(a.end, b.end));
    }
    return None;
}

pub fn solution(input: &str) -> (String, String) {
    let mut split = input.split_terminator("\n\n");
    let seeds: Vec<_> = split
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    let mut maps: Vec<_> = split
        .map(|m| {
            m.split_once(":\n")
                .unwrap()
                .1
                .split('\n')
                .map(|line| {
                    let mut s = line.split_whitespace();
                    let dst_start = s.next().unwrap().parse::<u64>().unwrap();
                    let src_start = s.next().unwrap().parse::<u64>().unwrap();
                    let length = s.next().unwrap().parse::<u64>().unwrap();
                    (src_start..src_start + length, dst_start..dst_start + length)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let part_1 = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |n, map| {
                map.iter()
                    .find(|(src, _)| src.contains(&n))
                    .and_then(|(src, dst)| Some(dst.start + (n - src.start)))
                    .unwrap_or(n)
            })
        })
        .min()
        .unwrap();

    for map in maps.iter_mut() {
        map.sort_unstable_by(|(src_a, _), (src_b, _)| src_a.start.cmp(&src_b.start));
    }

    fn min_location(
        range: &Range<u64>,
        map_idx: usize,
        maps: &[Vec<(Range<u64>, Range<u64>)>],
    ) -> u64 {
        if map_idx == maps.len() {
            return range.start;
        }

        let mut location = u64::MAX;
        let mut cursor = range.start;
        for (src, dst) in maps[map_idx].iter() {
            if let Some(overlap) = intersect(range, src) {
                if overlap.start > cursor {
                    location =
                        min(location, min_location(&(cursor..overlap.start), map_idx + 1, maps));
                }

                let r = overlap.start - src.start + dst.start..overlap.end - src.start + dst.start;
                location = min(location, min_location(&r, map_idx + 1, maps));
                cursor = overlap.end;
            }
        }

        if cursor < range.end {
            location = min(location, min_location(&(cursor..range.end), map_idx + 1, maps))
        }

        return location;
    }

    let mut part_2 = u64::MAX;
    for seed_range in seeds.chunks(2).map(|s| s[0]..s[0] + s[1]) {
        part_2 = min(part_2, min_location(&seed_range, 0, &maps));
    }

    return (part_1.to_string(), part_2.to_string());
}
