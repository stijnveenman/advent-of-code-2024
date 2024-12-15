use std::{thread::sleep, time::Duration};

use advent_of_code::{
    components::Point,
    grid::{char_grid::CharGrid, hash_grid::HashGrid, Grid},
};
use itertools::Itertools;
use ratatui::{
    crossterm::event::{self, poll, Event, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

advent_of_code::solution!(15);

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

fn parse(input: &str) -> (Point, HashGrid<'_, char>, Vec<Point>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = CharGrid::new(grid);

    let start = grid
        .entries()
        .find(|(_p, v)| *v == '@')
        .map(|(p, _v)| p)
        .unwrap();

    let grid = HashGrid::from_chargrid(grid, |c| match c {
        'O' => Some('O'),
        '#' => Some('#'),
        _ => None,
    });

    let moves = moves
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| match c {
            '<' => Point::LEFT,
            '>' => Point::RIGHT,
            'v' => Point::DOWN,
            '^' => Point::UP,
            c => panic!("unknown {}", c),
        })
        .collect_vec();

    (start, grid, moves)
}

fn try_move(point: &Point, dir: &Point, grid: &mut HashGrid<'_, char>) -> bool {
    if !grid.get(point).is_some_and(|c| *c == 'O') {
        return false;
    }

    let next = *point + *dir;

    if grid.contains(&next) && !try_move(&next, dir, grid) {
        return false;
    }

    let c = grid.remove(point).unwrap();
    grid.set(&next, c);

    true
}

fn try_move_2w(point: &Point, dir: &Point, grid: &mut HashGrid<'_, char>) -> bool {
    if grid.get(point).is_some_and(|c| *c == '#') {
        return false;
    }

    if grid.get(point).is_none() {
        return true;
    }

    if dir == &Point::RIGHT || dir == &Point::LEFT {
        // horizontal move

        let next = *point + (*dir * 2);

        if !try_move_2w(&next, dir, grid) {
            return false;
        }

        let c = grid.remove(&(*point + (*dir))).unwrap();
        grid.set(&(*point + *dir * 2), c);

        let c = grid.remove(point).unwrap();
        grid.set(&(*point + *dir), c);
    } else {
        // vertical move

        let b = grid.get(point).unwrap();
        let (left, right) = get_left_right_box(point, *b);

        // This will modify the grid if either of these fails but the other succeeds, so we need to
        // copy the grid before the original try_move_2w attempt and revert if it fails
        if !try_move_2w(&(left + *dir), dir, grid) || !try_move_2w(&(right + *dir), dir, grid) {
            return false;
        }

        let c = grid.remove(&left).unwrap();
        grid.set(&(left + *dir), c);

        let c = grid.remove(&right).unwrap();
        grid.set(&(right + *dir), c);
    }

    true
}

fn get_left_right_box(point: &Point, c: char) -> (Point, Point) {
    if c == '[' {
        (*point, *point + Point::RIGHT)
    } else {
        (*point + Point::LEFT, *point)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut pos, mut grid, moves) = parse(input);

    for dir in moves {
        let next = pos + dir;

        if grid.contains(&next) {
            if try_move(&next, &dir, &mut grid) {
                pos = next;
            }
        } else {
            pos = next;
        }
    }

    // grid.print(|_p, c| match c {
    //     Some(c) => c.to_string(),
    //     None => " ".to_string(),
    // });

    let result = grid
        .entries()
        .filter_map(|(p, c)| match c {
            'O' => Some(p),
            _ => None,
        })
        .map(|p| (p.y * 100 + p.x) as usize)
        .sum();

    Some(result)
}

fn expand(grid: HashGrid<'_, char>) -> HashGrid<'_, char> {
    let mut new_grid = HashGrid::new();

    for (point, c) in grid.entries() {
        match c {
            'O' => {
                new_grid.set(&Point::new(point.x * 2, point.y), '[');
                new_grid.set(&Point::new(point.x * 2 + 1, point.y), ']');
            }
            '#' => {
                new_grid.set(&Point::new(point.x * 2, point.y), '#');
                new_grid.set(&Point::new(point.x * 2 + 1, point.y), '#');
            }
            c => panic!("{} not handled", c),
        }
    }

    new_grid
}

pub fn part_two(input: &str) -> Option<usize> {
    let (pos, grid, moves) = parse(input);
    let mut grid = expand(grid);
    let mut pos = Point::new(pos.x * 2, pos.y);

    // grid.print(|p, c| {
    //     if *p == pos {
    //         return "@".into();
    //     }
    //
    //     match c {
    //         Some(c) => c.to_string(),
    //         None => " ".to_string(),
    //     }
    // });

    let mut terminal = ratatui::init();

    let mut paused = false;
    let mut single_step = false;
    let mut milis = 10;

    for dir in moves {
        if poll(Duration::from_secs(0)).is_ok_and(|e| e) {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('.') => single_step = true,
                    KeyCode::Up => milis += 1,
                    KeyCode::Down => milis = (milis - 1).max(1),
                    _ => {}
                }
            }
        }
        sleep(Duration::from_millis(milis));
        terminal
            .draw(|frame| {
                let instructions = Line::from(vec![
                    " Pause ".into(),
                    "<Space>".blue().bold(),
                    " Single step ".into(),
                    "<.>".blue().bold(),
                    " Speed ".into(),
                    "<Up>/<Down> ".blue().bold(),
                    " Quit ".into(),
                    "<Q> ".blue().bold(),
                ]);

                let block = Block::bordered()
                    .border_set(border::THICK)
                    .title_bottom(instructions.centered());
                let bounds = grid.bounds().1;

                let area = center(
                    frame.area(),
                    Constraint::Length(bounds.x as u16 + 3),
                    Constraint::Length(bounds.y as u16 + 3),
                );

                // +1 because our bounds are not the size, +2 for each border
                let rect = Rect::new(area.x, area.y, bounds.x as u16 + 3, bounds.y as u16 + 3);

                let s = grid.draw(|p, c| {
                    if *p == pos {
                        return "@".into();
                    }

                    match c {
                        Some(c) => c.to_string(),
                        None => " ".to_string(),
                    }
                });

                let paragraph = Paragraph::new(Text::from(s)).centered().block(block);

                frame.render_widget(paragraph, rect);
            })
            .unwrap();

        if paused && !single_step {
            continue;
        }
        single_step = false;

        let next = pos + dir;

        if grid.contains(&next) {
            let old = grid.clone();

            if try_move_2w(&next, &dir, &mut grid) {
                pos = next;
            } else {
                // Revert grid on failed move
                grid = old;
            }
        } else {
            pos = next;
        }
    }

    ratatui::restore();

    let result = grid
        .entries()
        .filter_map(|(p, c)| match c {
            '[' => Some(p),
            _ => None,
        })
        .map(|p| (p.y * 100 + p.x) as usize)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part2_example() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        let result = part_two(input);

        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
