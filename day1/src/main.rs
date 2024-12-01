use std::{collections::HashMap, error::Error, fs::File, io::{BufRead, BufReader}};


fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input1.txt")?;
    let reader = BufReader::new(f);

    let mut left_ids = Vec::new();
    let mut right_ids = Vec::new();
    let mut id_counts = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        let mut parts = line.split_whitespace().map(|p| p.parse::<i32>());

        if let (Some(Ok(id1)), Some(Ok(id2))) = (parts.next(), parts.next()) {
            left_ids.push(id1);
            right_ids.push(id2);
            *id_counts.entry(id2).or_insert(0) += 1;
        }
    }

    left_ids.sort_unstable();
    right_ids.sort_unstable();

    // part 1
    let sum: i32 = left_ids
        .iter()
        .zip(right_ids)
        .map(|(id1, id2)| (id1 - id2).abs())
        .sum();

    println!("The sum is: {}", sum);

    // part 2
    let sum: i32 = left_ids
        .iter()
        .filter_map(|&id| id_counts.get(&id).map(|&count| id * count))
        .sum();

    println!("The sum is: {}", sum);

    Ok(())
}
