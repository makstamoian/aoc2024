fn part_one() {
    let input = include_str!("./input.txt");

    let mut group_one_ids: Vec<u32> = [].to_vec();
    let mut group_two_ids: Vec<u32> = [].to_vec();

    for line in input.lines() {
        let line_ids: Vec<&str> = line.split_whitespace().collect();
        group_one_ids.push(line_ids[0].parse().unwrap());
        group_two_ids.push(line_ids[1].parse().unwrap())
    }

    group_one_ids.sort();
    group_two_ids.sort();

    let mut sum: u32 = 0;

    for i in 0..group_one_ids.len() {
        sum += group_one_ids[i].abs_diff(group_two_ids[i]);
    }

    println!("Part one: {}", sum);
}

fn part_two() {
    let input = include_str!("./input.txt");

    let mut group_one_ids: Vec<u32> = [].to_vec();
    let mut group_two_ids: Vec<u32> = [].to_vec();

    for line in input.lines() {
        let line_ids: Vec<&str> = line.split_whitespace().collect();
        group_one_ids.push(line_ids[0].parse().unwrap());
        group_two_ids.push(line_ids[1].parse().unwrap())
    }

    let mut similarity_score: u32 = 0;

    for group_one_id in group_one_ids {
        similarity_score += group_one_id * group_two_ids.iter().filter(|x| x == &&group_one_id).count() as u32;
    }

    println!("Part two: {}", similarity_score);
}

fn main() {
    part_one();
    part_two();
}
