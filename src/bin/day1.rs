use std::collections::BTreeMap;

fn main() {
    let example_data = include_str!("../input/day1_example.txt");
    let data = include_str!("../input/day1.txt");

    let example_lists = parse_data_into_two_sorted_lists(example_data);
    let lists = parse_data_into_two_sorted_lists(data);

    assert_eq!(calculate_difference(example_lists.clone()), 11);
    println!("1: {}", calculate_difference(lists.clone()));

    assert_eq!(calculate_similarity(example_lists.clone()), 31);
    println!("2: {}", calculate_similarity(lists));
}

fn parse_data_into_two_sorted_lists(data: &str) -> (Vec<usize>, Vec<usize>) {
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in data.lines() {
        let mut nums = line.split_whitespace();
        first.push(nums.next().unwrap().parse().unwrap());
        second.push(nums.next().unwrap().parse().unwrap());
    }

    first.sort();
    second.sort();

    (first, second)
}

fn calculate_difference((first, second): (Vec<usize>, Vec<usize>)) -> usize {
    first
        .into_iter()
        .zip(second)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn calculate_similarity((first, second): (Vec<usize>, Vec<usize>)) -> usize {
    let counts = second.into_iter().fold(BTreeMap::new(), |mut map, n| {
        *map.entry(n).or_insert(0) += 1;
        map
    });

    first
        .into_iter()
        .map(|n| n * counts.get(&n).unwrap_or(&0))
        .sum()
}
