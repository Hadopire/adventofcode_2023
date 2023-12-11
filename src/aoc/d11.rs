pub fn solution(input: &str) -> (String, String) {
    let width = input.as_bytes().iter().position(|c| *c == b'\n').unwrap();
    let height = (input.len() + 1) / (width + 1);

    let mut empty_columns = vec![true; width];
    let mut empty_rows = vec![true; height];
    let mut galaxies : Vec<(i32, i32)> = vec![];
    let mut big_galaxies : Vec<(i64, i64)> = vec![];

    for (y, line) in input.split_terminator('\n').enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c == b'#' {
                empty_columns[x] = false;
                empty_rows[y] = false;
                galaxies.push((x as i32, y as i32));
                big_galaxies.push((x as i64, y as i64));
            }
        }
    }

    for (col, _) in empty_columns.iter().enumerate().rev().filter(|(_, &empty)| empty == true) {
        for i in 0..galaxies.len() {
            if galaxies[i].0 > col as i32 {
                galaxies[i].0 += 1;
                big_galaxies[i].0 += 999999;
            }
        }
    }
    
    for (row, _) in empty_rows.iter().enumerate().rev().filter(|(_, &empty)| empty == true) {
        for i in 0..galaxies.len() {
            if galaxies[i].1 > row as i32 {
                galaxies[i].1 += 1;
                big_galaxies[i].1 += 999999;
            }
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            part_1 += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
            part_2 += (big_galaxies[i].0 - big_galaxies[j].0).abs() + (big_galaxies[i].1 - big_galaxies[j].1).abs();
        }
    }

    return (part_1.to_string(), part_2.to_string());
}