fn main() {
    let example1_data = include_str!("../input/day3_example1.txt");
    let example2_data = include_str!("../input/day3_example2.txt");
    let data = include_str!("../input/day3.txt");

    assert_eq!(extract_muls(example1_data, false), 161);
    println!("1: {}", extract_muls(data, false)); // 159892596

    assert_eq!(extract_muls(example2_data, true), 48);
    println!("2: {}", extract_muls(data, true)); // 92626942
}

fn extract_muls(data: &str, verify_do: bool) -> usize {
    let data = data.split("mul(").skip(1).collect::<Vec<_>>();

    let mut muls = 0;
    let mut mul_enabled = true;

    for section in data {
        let Some((first_number, second_number)) = get_mult_numbers(section) else {
            continue;
        };

        if !verify_do || mul_enabled {
            muls += first_number * second_number;
        }

        mul_enabled = is_mul_enabled(section).unwrap_or(mul_enabled);
    }

    muls
}

fn get_mult_numbers(section: &str) -> Option<(usize, usize)> {
    let (number, rest) = section.split_once(",")?;
    let first_number = number.parse::<usize>().ok()?;
    let (number, _) = rest.split_once(")")?;
    let second_number = number.parse::<usize>().ok()?;

    Some((first_number, second_number))
}

fn is_mul_enabled(section: &str) -> Option<bool> {
    match (
        get_last_len(section, "do()"),
        get_last_len(section, "don't()"),
    ) {
        (Some(do_last), Some(dont_last)) if do_last > dont_last => Some(false),
        (Some(do_last), Some(dont_last)) if do_last < dont_last => Some(true),
        (Some(_), None) => Some(true),
        (None, Some(_)) => Some(false),
        _ => None,
    }
}

fn get_last_len(section: &str, keyword: &str) -> Option<usize> {
    section.split(keyword).last().map(|s| s.len())
}
