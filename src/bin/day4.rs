fn main() {
    let example_data = include_str!("../input/day4_example.txt");
    let data = include_str!("../input/day4.txt");

    assert_eq!(number_of_xmas(example_data), 18);
    let answer1 = number_of_xmas(data);
    println!("1: {answer1}");
    assert_eq!(answer1, 2336);

    assert_eq!(number_of_x_mas(example_data), 9);
    let answer2 = number_of_x_mas(data);
    println!("2: {answer2}");
    assert_eq!(answer2, 1831);
}

fn number_of_x_mas(data: &str) -> usize {
    let chars = get_chars(data);
    let mut count = 0;

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            count += x_max_count(&chars, i, j);
        }
    }

    count
}

fn x_max_count(chars: &[Vec<char>], i: usize, j: usize) -> usize {
    if chars[i][j] != 'A' || i == 0 || i == chars.len() - 1 || j == 0 || j == chars[i].len() - 1 {
        return 0;
    }

    let corners = [
        chars[i - 1][j - 1], // top left
        chars[i - 1][j + 1], // top right
        chars[i + 1][j - 1], // bottom left
        chars[i + 1][j + 1], // bottom right
    ];

    matches!(
        corners,
        ['S', 'S', 'M', 'M'] | ['M', 'M', 'S', 'S'] | ['M', 'S', 'M', 'S'] | ['S', 'M', 'S', 'M']
    ) as usize
}

fn number_of_xmas(data: &str) -> usize {
    let chars = get_chars(data);
    let mut count = 0;

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            count += xmax_count(&chars, i, j);
        }
    }

    count
}

const XMAS: &[char] = &['M', 'A', 'S'];

fn xmax_count(chars: &[Vec<char>], i: usize, j: usize) -> usize {
    if chars[i][j] != 'X' {
        return 0;
    }

    [
        check_horizontal_forward(&chars[i], j),
        check_horizontal_backward(&chars[i], j),
        check_vertical_down(chars, i, j),
        check_vertical_up(chars, i, j),
        check_diagonal_down_forward(chars, &chars[i], i, j),
        check_diagonal_down_backward(chars, &chars[i], i, j),
        check_diagonal_up_forward(chars, &chars[i], i, j),
        check_diagonal_up_backward(chars, i, j),
    ]
    .iter()
    .filter(|&&x| x)
    .count()
}

fn check_horizontal_forward(line: &[char], j: usize) -> bool {
    line.iter()
        .skip(j + 1)
        .take(3)
        .cloned()
        .collect::<Vec<_>>()
        .as_slice()
        == XMAS
}

fn check_horizontal_backward(line: &[char], j: usize) -> bool {
    line.iter()
        .rev()
        .skip(line.len() - j)
        .take(3)
        .cloned()
        .collect::<Vec<_>>()
        .as_slice()
        == XMAS
}

fn check_vertical_down(chars: &[Vec<char>], i: usize, j: usize) -> bool {
    chars
        .iter()
        .skip(i + 1)
        .take(3)
        .map(|line| line[j])
        .collect::<Vec<_>>()
        .as_slice()
        == XMAS
}

fn check_vertical_up(chars: &[Vec<char>], i: usize, j: usize) -> bool {
    chars
        .iter()
        .rev()
        .skip(chars.len() - i)
        .take(3)
        .map(|line| line[j])
        .collect::<Vec<_>>()
        .as_slice()
        == XMAS
}

fn check_diagonal_down_forward(chars: &[Vec<char>], line: &[char], i: usize, j: usize) -> bool {
    let mut next_line = i + 1;
    let mut next_char = j + 1;
    let mut index_xmas = 0;
    while next_line < chars.len()
        && next_char < line.len()
        && chars[next_line][next_char] == XMAS[index_xmas]
    {
        if index_xmas == 2 {
            return true;
        }

        index_xmas += 1;
        next_line += 1;
        next_char += 1;
    }
    false
}

fn check_diagonal_down_backward(chars: &[Vec<char>], line: &[char], i: usize, j: usize) -> bool {
    let mut next_line = i + 1;
    let mut next_char = j as isize - 1;
    let mut index_xmas = 0;
    while next_line < chars.len()
        && next_char >= 0
        && chars[next_line][next_char as usize] == XMAS[index_xmas]
    {
        if index_xmas == 2 {
            return true;
        }

        next_line += 1;
        next_char -= 1;
        index_xmas += 1;
    }
    false
}

fn check_diagonal_up_forward(chars: &[Vec<char>], line: &[char], i: usize, j: usize) -> bool {
    let mut next_line = i as isize - 1;
    let mut next_char = j + 1;
    let mut index_xmas = 0;
    while next_line >= 0
        && next_char < line.len()
        && chars[next_line as usize][next_char] == XMAS[index_xmas]
    {
        if index_xmas == 2 {
            return true;
        }

        next_line -= 1;
        next_char += 1;
        index_xmas += 1;
    }
    false
}

fn check_diagonal_up_backward(chars: &[Vec<char>], i: usize, j: usize) -> bool {
    let mut next_line = i as isize - 1;
    let mut next_char = j as isize - 1;
    let mut index_xmas = 0;
    while next_line >= 0
        && next_char >= 0
        && chars[next_line as usize][next_char as usize] == XMAS[index_xmas]
    {
        if index_xmas == 2 {
            return true;
        }

        next_line -= 1;
        next_char -= 1;
        index_xmas += 1;
    }
    false
}

fn get_chars(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}
