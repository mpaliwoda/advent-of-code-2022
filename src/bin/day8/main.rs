use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/day8/input.txt")
        .expect("I don't think I'm gonna last till day 25");

    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|ch| ch != &"")
                .map(|height| height.parse().unwrap())
                .collect()
        })
        .collect();

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<usize>>) {
    let number_of_rows = grid.len();
    let number_of_cols = grid.last().unwrap().len();
    let mut visible = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if i == 0 || j == 0 || i == number_of_rows - 1 || j == number_of_cols - 1 {
                visible += 1;
                continue;
            }

            if (0..i).fold(true, |acc, x| acc && grid[x][j] < *height)
                || (0..j).fold(true, |acc, x| acc && grid[i][x] < *height)
                || (i + 1..number_of_rows)
                    .rev()
                    .fold(true, |acc, x| acc && grid[x][j] < *height)
                || (j + 1..number_of_cols)
                    .rev()
                    .fold(true, |acc, x| acc && grid[i][x] < *height)
            {
                visible += 1;
            }
        }
    }
    println!("Part 1: {}", visible);
}

fn part2(grid: &Vec<Vec<usize>>) {
    let mut scenic_score = 0;
    let number_of_rows = grid.len();
    let number_of_cols = grid.last().unwrap().len();

    for (i, row) in grid.iter().enumerate() {
        if i == 0 || i == number_of_rows - 1 {
            continue;
        }
        for (j, height) in row.iter().enumerate() {
            if j == 0 || j == number_of_cols - 1 {
                continue;
            }
            let mut current_score = 1;

            let mut up = 0;
            for x in (0..i).rev() {
                up += 1;
                if grid[x][j] >= *height {
                    break;
                }
            }
            current_score *= up;

            let mut down = 0;
            for x in i + 1..number_of_rows {
                down += 1;
                if grid[x][j] >= *height {
                    break;
                }
            }
            current_score *= down;

            let mut left = 0;
            for x in (0..j).rev() {
                left += 1;
                if grid[i][x] >= *height {
                    break;
                }
            }
            current_score *= left;

            let mut right = 0;
            for x in j + 1..number_of_cols {
                right += 1;
                if grid[i][x] >= *height {
                    break;
                }
            }
            current_score *= right;

            if current_score > scenic_score {
                scenic_score = current_score;
            }
        }
    }
    println!("Part 2: {}", scenic_score);
}
