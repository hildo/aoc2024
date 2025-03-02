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

    fn current_position_on_map(&self) -> bool {
        self.position_on_map(&self.current_pose)
    }

    fn do_navigate<F: FnMut(Pose, Pose), G: FnMut(Pose), H: FnMut(Pose, Pose)>(&mut self, mut replace_fn: F, mut obstruct_fn: G, mut navigate_fn: H) {
        let original_pose = self.current_pose;
        while self.current_position_on_map() {
            let mut replace_current = true;
            let next_position = self.current_pose.next_candidate();
            if self.position_on_map(&next_position) {
                // The target is still on the map.  Check to see if there is an obstruction
                match self.rows[next_position.x as usize].chars().nth(next_position.y as usize).unwrap() {
                    '#' => {
                        // There is an obstruction.  Move ninety degrees
                        self.current_pose.turn_ninety_degrees();
                        replace_current = false;
                        obstruct_fn(next_position);
                    },
                    _ => {
                        // No obstruction, move into the next cell
                        navigate_fn(self.current_pose, next_position)
                    }
                }
            }
            if replace_current {
                replace_fn(self.current_pose, next_position);
                self.current_pose = next_position;
            }
        }
        self.current_pose = original_pose;
    }

    fn count_distinct_positions(&mut self) -> usize {
        let mut visited_cells: Vec<Pose> = Vec::new();
        self.do_navigate(
            |current, next| visited_cells.push(current),
            |obstruction| {},
            |current, next| {}
        );
        visited_cells.into_iter().collect::<HashSet<Pose>>().into_iter().len()
    }

    fn looping_obstacle_candidates(&mut self) -> usize {
        let mut encountered_obstructions: Vec<Pose> = Vec::new();
        // Navigate the path, finding obstructions
        self.do_navigate(
            |current, next| {},
            |obstruction| encountered_obstructions.push(obstruction),
            |current, next| {}
        );
        // Now that we have obstructions, find obstruction candidates that would cause a loop
        let mut looping_obstruction_candidates: Vec<(Pose, Pose)> = Vec::new();
        self.do_navigate(
            |current, next| {},
            |obstruction| {},
            |current, next| {
                match current.heading {
                    Heading::Up => {
                        // Look to the right.  See if we have encountered any obstructions already
                        match encountered_obstructions.iter().find(|obs| obs.x == current.x && obs.y > current.y) {
                            Some(p) => looping_obstruction_candidates.push((current, next)),
                            _ => { 
                                // nothing 
                            }
                        };
                    },
                    Heading::Down => {
                        // Look to the right.  See if we have encountered any obstructions already
                        match encountered_obstructions.iter().find(|obs| obs.x == current.x && obs.y < current.y) {
                            Some(p) => looping_obstruction_candidates.push((current, next)),
                            _ => { 
                                // nothing 
                            }
                        };
                    },
                    Heading::Left => {
                        match encountered_obstructions.iter().find(|obs| obs.x < current.x && obs.y == current.y) {
                            Some(p) => looping_obstruction_candidates.push((current, next)),
                            _ => { 
                                // nothing 
                            }
                        };
                    },
                    Heading::Right => {
                        match encountered_obstructions.iter().find(|obs| obs.x > current.x && obs.y == current.y) {
                            Some(p) => looping_obstruction_candidates.push((current, next)),
                            _ => { 
                                // nothing 
                            }
                        };
                    },
                }
            }
        );

        let mut new_obstructions: Vec<Pose> = Vec::new();

        // for each candidate, update the map with the new obstacle and see if a loop occurs
        for candidate_pair in looping_obstruction_candidates {
            let mut new_rows = self.rows.clone();
            let mut row_to_change = new_rows[candidate_pair.1.x as usize].clone();
            row_to_change.replace_range(candidate_pair.1.y as usize..candidate_pair.1.y as usize +1, "#");

            let mut new_map = Map {rows: new_rows, current_pose: self.current_pose };
            let mut in_a_loop = false;
            
            let mut counter = 0;
            new_map.do_navigate(
                |_, _| {},
                |_| {},
                |current, next| {
                    // listen for whether the path naviates through the point twice
                    if current == candidate_pair.0 {
                        counter += 1;
                    }
                });

            if counter > 1 {
                new_obstructions.push(candidate_pair.1);
            }
        }
        
        new_obstructions.into_iter().collect::<HashSet<Pose>>().into_iter().len()
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

    #[test]
    fn test_part_two_simple() {
        let mut map = load_map("./src/resources/day06_simple.txt");
        assert_eq!(6, map.looping_obstacle_candidates());
        // let candidates = map.looping_obstacle_candidates();
    }

    // #[test]
    // fn test_part_two() {
    //     let mut map = load_map("./src/resources/day06_input.txt");
    //     println!("{}", map.count_looping_obstacles());
    //     // 2092 fails ... too high
    //     // assert_eq!(6, map.count_looping_obstacles());
    // }

}