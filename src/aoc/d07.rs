const CARDS : [u8; 13]  = [b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A'];
const CARDS_WITH_JOKER : [u8; 13]  = [b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A'];

fn hand_score(hand: &str, with_joker: bool) -> u64 {
    let cards = with_joker.then(|| CARDS_WITH_JOKER).unwrap_or(CARDS);

    let mut score = hand.as_bytes().iter().enumerate().fold(0, |acc, (idx, card)| acc + (cards.iter().position(|e| e == card).unwrap() << (4-idx) * 5)) as u64;
    let mut dups : Vec<(u8, u8)>  = Vec::with_capacity(5);

    for card in hand.as_bytes() {
        if let Some(idx) = dups.iter().position(|(c,_)| c == card) {
            dups[idx].1 += 1;
        } else {
            dups.push((*card, 1));
        }
    }

    if with_joker {
        dups.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        if let Some(j_idx) = dups.iter().position(|(c,_)| *c == b'J') {
            let joker_count = dups[j_idx].1;
            if dups.len() > 1 {
               dups.remove(j_idx);
               dups[0].1 += joker_count; 
            }
        }
    }

    score += match dups.len() {
        1 => 6,
        2 => if dups.iter().find(|(_,n)| *n == 4).is_some() { 5 } else { 4 },
        3 => if dups.iter().find(|(_,n)| *n == 3).is_some() { 3 } else { 2 },
        4 => 1, 
        _ => 0,
    } << 25;
    return score;
}

pub fn solution(input: &str) -> (String, String) {
    let mut hands : Vec<_> = input.split_terminator('\n').map(|s| {
        let split = s.split_once(' ').unwrap();
        (split.0, split.1.parse::<u32>().unwrap(), hand_score(split.0, false), hand_score(split.0, true))
    }).collect();
    
    hands.sort_unstable_by(|a, b| a.2.cmp(&b.2));
    let part_1 = hands.iter().enumerate().fold(0, |acc, (idx, (_, bet, _, _))| (idx+1) as u32 * *bet + acc);
    hands.sort_unstable_by(|a, b| a.3.cmp(&b.3));
    let part_2 = hands.iter().enumerate().fold(0, |acc, (idx, (_, bet, _, _))| (idx+1) as u32 * *bet + acc);

    return (part_1.to_string(), part_2.to_string());
}