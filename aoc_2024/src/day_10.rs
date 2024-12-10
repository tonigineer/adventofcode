use common::{grid::Grid, Answer, Solution};
use std::collections::VecDeque;

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> &'static str {
        "Hoof It"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut ans = 0;
    let grid: Grid<isize> = Grid::from(input);

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c] == 0 {
                ans += find_trails(&grid, &c, &r, part2);
            }
        }
    }
    ans
}

fn find_trails(grid: &Grid<isize>, c: &usize, r: &usize, part2: bool) -> u32 {
    let mut trails = 0;

    let mut q: VecDeque<(isize, isize)> = VecDeque::new();
    q.push_back((*c as isize, *r as isize));

    while !q.is_empty() {
        let (c, r) = q.pop_front().unwrap();

        if grid.data[r as usize][c as usize] == 9 {
            trails += 1;
            continue;
        }

        for (nc, nr) in grid.adjacent_cardinals(c as usize, r as usize) {
            if grid.data[r as usize][c as usize] + 1 == grid.data[(nr) as usize][nc] {
                if !q.contains(&(nc as isize, nr as isize)) || part2 {
                    q.push_back((nc as isize, nr as isize));
                }
            }
        }
    }
    trails
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn example_part1() {
        assert_eq!(Day10::part1(&Day10, SAMPLE), common::Answer::Number(36));
    }

    #[test]
    fn example_part2() {
        assert_eq!(Day10::part2(&Day10, SAMPLE), common::Answer::Number(81));
    }
}
