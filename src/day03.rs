use std::collections::HashMap;

/**
 */
pub fn solve(path: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(path)?;
    let mut lines = file.lines();

    let line_len = lines.next().unwrap().len() + 1;
    static RE: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| regex::Regex::new(r#"(\d+)"#).unwrap());

    let mut star_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let caps = RE.captures_iter(&file);
    let s1: usize = caps
        .map(|cap| {
            let cap = cap.get(1).unwrap();
            let start_idx = cap.start();
            let length = cap.len();

            let (is_part_number, star_idxs) =
                check_around(start_idx, length, file.as_bytes(), line_len);

            if is_part_number {
                let res = cap.as_str().parse().unwrap();
                star_idxs.iter().for_each(|&x| {
                    star_map
                        .entry(x)
                        .and_modify(|sm| sm.push(res))
                        .or_insert(vec![res]);
                });
                res
            } else {
                0
            }
        })
        .sum();

    let s2: usize = star_map
        .iter()
        .filter(|(_, x)| x.len() > 1)
        .map(|(_, idxs)| idxs.iter().fold(1, |acc, i| acc * i))
        .sum();

    Ok((s1.to_string(), s2.to_string()))
}

fn check_around(idx: usize, length: usize, buffer: &[u8], row_len: usize) -> (bool, Vec<usize>) {
    let row = idx / row_len;
    let max_rows = buffer.len() / row_len;
    let last_row = max_rows - 1;
    let col = idx % row_len;
    let coord_to_idx = |row: usize, col: usize| row * row_len + col;
    let restrict_col = |c: usize| if c >= row_len - 1 { row_len - 2 } else { c };

    let mut is_part_number = false;
    let mut star_idxs = Vec::new();

    for r in row.saturating_sub(1)..=(row + 1).min(last_row) {
        for c in col.saturating_sub(1)..=restrict_col(col + length) {
            debug_assert!(col < row_len);
            let idx = coord_to_idx(r, c);
            let symb = check_symbol(&(buffer[idx] as char));
            match symb {
                SymbolType::Star => {
                    star_idxs.push(idx);
                    is_part_number = true;
                }
                SymbolType::Symbol => {
                    is_part_number = true;
                }
                SymbolType::None => {}
            }
        }
    }

    (is_part_number, star_idxs)
}

enum SymbolType {
    Star,
    Symbol,
    None,
}

fn check_symbol(c: &char) -> SymbolType {
    match c {
        '*' => SymbolType::Star,
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => SymbolType::None,
        '\n' | '\r' => panic!("newline in input"),
        _ => SymbolType::Symbol,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn evaluate_1() {
        let actual = super::solve("testdata/day03.txt").unwrap();
        assert_eq!((4361.to_string(), 467835.to_string()), actual);
    }
}
