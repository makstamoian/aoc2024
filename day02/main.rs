fn check_report(report: &Vec<u32>) -> bool {
    let mut prev_diff = report[0] as i32 - report[1] as i32;

    for i in 0..report.len() - 1 {
        let abs_diff = report[i].abs_diff(report[i + 1]);
        if abs_diff > 3 || abs_diff < 1 {
            return false;
        }

        let diff = report[i] as i32 - report[i + 1] as i32;

        if diff > 0 {
            if prev_diff >= 0 {
                prev_diff = diff;
                continue;
            }

            return false
        }

        if diff < 0 {
            if prev_diff <= 0 {
                prev_diff = diff;
                continue;
            }

            return false;
        }

    }

    return true;
}

fn part_one() {
    let input = include_str!("input.txt");

    let mut safe_reports: u32 = 0;

    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();

        if check_report(&report) {
            safe_reports += 1;
        }
    }

    println!("Part one: {}", safe_reports);
}

fn part_two() {
    let input = include_str!("input.txt");

    let mut safe_reports: u32 = 0;

    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();

        if check_report(&report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);

                if check_report(&new_report) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }


    println!("Part two: {}", safe_reports);

}

fn main() {
    part_one();
    part_two();
}
