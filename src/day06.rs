use std::{collections::HashSet, hash::{Hash, Hasher}};

use crate::helpers;

fn load_input(input_file_name: &str) -> Vec<String> {
    if let Ok(lines) = helpers::read_lines(input_file_name) {
        lines.flatten().collect()
    } else{
        Vec::new()
    }
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Heading {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy)]
#[derive(Clone)]
struct Pose {
    x: isize,
    y: isize,
    heading: Heading
}

impl Pose {

    fn same_location(&self, other: &Pose) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn turn_ninety_degrees(&mut self) {
        self.heading = match self.heading {
            Heading::Up => Heading::Right,
            Heading::Right => Heading::Down,
            Heading::Down => Heading::Left,
            Heading::Left => Heading::Up
        }
    }

    fn next_candidate(&self) -> Pose {
        match self.heading {
            Heading::Up => Pose{x: self.x - 1, y: self.y, heading: Heading::Up},
            Heading::Left => Pose{x: self.x, y: self.y - 1, heading: Heading::Left},
            Heading::Down => Pose{x: self.x + 1, y: self.y, heading: Heading::Down},
            Heading::Right => Pose{x: self.x, y: self.y + 1, heading: Heading::Right}
        }
    }

}

impl PartialEq for Pose {

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pose {

}

impl Hash for Pose {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

struct Map {
    rows: Vec<String>,
    current_pose: Pose
}

impl Map {

    fn position_on_map(&self, pose: &Pose) -> bool {
        if pose.x < 0 || pose.y < 0 {
            return false;
        }

        pose.x < self.rows.len() as isize
            && pose.y < self.rows[self.current_pose.x as usize].len() as isize
    }

    fn current_posiion_on_map(&self) -> bool {
        self.position_on_map(&self.current_pose)
    }

    fn count_distinct_positions(&mut self) -> usize {
        // println!("Starting at ({},{}) {:?}", self.current_pose.x, self.current_pose.y, self.current_pose.heading);
        let mut visited_cells: Vec<Pose> = Vec::new();

        while self.current_posiion_on_map() {
            let mut replace_current = true;
            let next_position = self.current_pose.next_candidate();
            if self.position_on_map(&next_position) {
                // The target is still on the map.  Check to see if there is an obstruction
                match self.rows[next_position.x as usize].chars().nth(next_position.y as usize).unwrap() {
                    '#' => {
                        // There is an obstruction.  Move ninety degrees
                        self.current_pose.turn_ninety_degrees();
                        replace_current = false;
                        // println!("Turning right at ({},{}) to {:?}", self.current_pose.x, self.current_pose.y, self.current_pose.heading);
                    },
                    _ => {
                        // No obstruction, moce into the next cell
                    }
                }
            }
            if replace_current {
                visited_cells.push(self.current_pose);
                self.current_pose = next_position;
            }
        }
        visited_cells.into_iter().collect::<HashSet<Pose>>().into_iter().len()
    }

}

fn locate_pos_and_heading(lines: &Vec<String>) -> Pose {
    let mut row: usize = 0;
    let mut col: usize = 0;

    while row < lines.len() {
        let line = &lines[row];
        let position_option = line.chars().position(|c| c == '^');
        match position_option {
            Some(idx) => {
                col = idx;
                break;
            },
            _ => row += 1
            
        }
    }

    Pose{x: row as isize, y: col as isize, heading: Heading::Up}
}


fn load_map(file_name: &str) -> Map {
    let lines = load_input(file_name);
    let starting_position = locate_pos_and_heading(&lines);
    Map {
        rows: lines,
        current_pose: starting_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut map = load_map("./src/resources/day06_simple.txt");
        assert_eq!(41, map.count_distinct_positions());
    }

    #[test]
    fn test_part_one() {
        let mut map = load_map("./src/resources/day06_input.txt");
        assert_eq!(5162, map.count_distinct_positions());
    }

    #[test]
    fn test_pose_next_candidate() {
        let my_pose = Pose{x: 1, y: 3, heading: Heading::Up};
        let next_pose = my_pose.next_candidate();
        assert!(!my_pose.same_location(&next_pose));
        assert_eq!(0, next_pose.x);
    }
}