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
    let re = Regex::new(r"mul\((?<number1>\d{1,3}),(?<number2>\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut mul_enabled: bool = true;
    let mut sum: u32 = 0;

    for cap in re.captures_iter(input) {
        if let Some(_) = cap.get(1) {
            if mul_enabled {
                sum += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
            }
        } else if cap.get(0).unwrap().as_str() == "do()" {
            mul_enabled = true;
        } else if cap.get(0).unwrap().as_str() == "don't()" {
            mul_enabled = false;
        }
    }

    println!("Part two: {}", sum);
    
}

fn main () {
    part_one();
    part_two();
}