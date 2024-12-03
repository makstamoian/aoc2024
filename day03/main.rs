use regex::Regex;

fn part_one() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"mul\((?<number1>\d{1,3}),(?<number2>\d{1,3})\)").unwrap();

    let mut sum: u32 = 0;

    for (_, [number1, number2]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += number1.parse::<u32>().unwrap() * number2.parse::<u32>().unwrap();
    }

    println!("Part one: {}", sum);
}

fn part_two() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"mul\((?<number1>\d{1,3}),(?<number2>\d{1,3})\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();

    let mut sum: u32 = 0;

    let mut matches = re.find_iter(input);
    let mut dont_matches = dont_re.find_iter(input);
    let mut do_matches = do_re.find_iter(input);

    while let Some(dont) = dont_matches.next() {
        
    }
}

fn main () {
    part_one();
    part_two();
}