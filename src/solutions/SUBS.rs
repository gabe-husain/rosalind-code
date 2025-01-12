pub fn SUBS(s: &str) -> Vec<usize> {
    let mut seq_subs = s.split("\n")
        .filter(|s| !s.is_empty());
    let seq = seq_subs.next().unwrap();
    let sub = seq_subs.next().unwrap();

    let mut positions: Vec<usize> = Vec::new();
    let mut i = 0;
    while let Some(pos) = seq[i..].find(sub) {
        positions.push(pos + i + 1);
        i += pos + 1;
    }
    positions
}