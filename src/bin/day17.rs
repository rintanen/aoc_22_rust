use std::collections::HashSet;

#[derive(Debug)]
enum Tetromino {
    Hyphen((i32, i32)),
    Plus((i32, i32)),
    RightAngle((i32, i32)),
    Pipe((i32, i32)),
    Square((i32, i32)),
}
const N_TETROMINOS: usize = 5;

impl Tetromino {
    fn move_by_jet_stream(&mut self, jet_stream: &i32) {
        match self {
            Tetromino::Hyphen((x, _)) => *x = (*x + jet_stream).max(0).min(3),
            Tetromino::Plus((x, _)) => *x = (*x + jet_stream).max(1).min(5),
            Tetromino::RightAngle((x, _)) => *x = (*x + jet_stream).max(0).min(4),
            Tetromino::Pipe((x, _)) => *x = (*x + jet_stream).max(0).min(6),
            Tetromino::Square((x, _)) => *x = (*x + jet_stream).max(0).min(5),
        }
    }

    fn move_down(&mut self) {
        match self {
            Tetromino::Hyphen((_, y))
            | Tetromino::Plus((_, y))
            | Tetromino::RightAngle((_, y))
            | Tetromino::Pipe((_, y))
            | Tetromino::Square((_, y)) => *y -= 1,
        }
    }

    fn collision_check(&self, contact_level: &Vec<i32>) -> bool {
        match self {
            Tetromino::Hyphen((x, y)) => {
                let points_of_impact = vec![(*x, y-1), (*x+1, y-1), (*x+2, y-1), (*x+3, y-1)];
                check_collision(&points_of_impact, contact_level)
            },
            Tetromino::Plus((x, y)) => {
                let points_of_impact = vec![(*x, y-1), (*x-1, *y), (*x+1, *y)];
                check_collision(&points_of_impact, contact_level)
            },
            Tetromino::RightAngle((x, y)) => {
                let points_of_impact = vec![(*x, *y-1), (*x+1, *y-1), (*x+2, *y-1)];
                check_collision(&points_of_impact, contact_level)
            },
            Tetromino::Pipe((x, y)) => {
                let points_of_impact = vec![(*x, *y-1)];
                check_collision(&points_of_impact, contact_level)
            }
            Tetromino::Square((x, y)) => {
                let points_of_impact = vec![(*x, *y-1), (*x+1, *y-1)];
                check_collision(&points_of_impact, contact_level)
            }
        }
    }

    fn new_points_of_contact(&self) -> Vec<(i32, i32)> {
        // update the contact level vector with these values
        match self {
            Tetromino::Hyphen((x, y)) => vec![(*x, *y), (*x+1, *y), (*x+2, *y), (*x+3, *y)],
            Tetromino::Plus((x, y)) => vec![(*x, y+2), (*x-1, *y+1), (*x+1, *y+1)],
            Tetromino::RightAngle((x, y)) => vec![(*x, *y), (*x+1, *y), (*x+2, *y+2)],
            Tetromino::Pipe((x, y)) => vec![(*x, *y+3)],
            Tetromino::Square((x, y)) => vec![(*x, *y+1), (*x+1, *y+1)],
        }
    }

    fn all_blocks(&self) -> Vec<(i32, i32)> {
        match self {
            Tetromino::Hyphen((x, y)) => vec![(*x, *y), (*x+1, *y), (*x+2, *y), (*x+3, *y)],
            Tetromino::Plus((x, y)) => vec![(*x, *y), (*x-1, *y+1), (*x+1, *y+1), (*x, *y+1), (*x, *y+2)],
            Tetromino::RightAngle((x, y)) => vec![(*x, *y), (*x+1, *y), (*x+2, *y), (*x+2, *y+1), (*x+2, *y+2)],
            Tetromino::Pipe((x, y)) => vec![(*x, *y), (*x, *y+1), (*x, *y+2), (*x, *y+3)],
            Tetromino::Square((x, y)) => vec![(*x, *y), (*x+1, *y), (*x, *y+1), (*x+1, *y+1)],
        }
    }
}

fn check_collision(points: &[(i32, i32)], contact_level: &Vec<i32>) -> bool {
    for &(x, y) in points.iter() {
        let contact_value = contact_level[x as usize];
        if y == contact_value {
            return true;
        }
    }
    false
}

struct ContactLevel {
    level: Vec<i32>,
}

impl ContactLevel {
    fn new() -> Self {
        Self {
            level: vec![0; 7],
        }
    }

    fn max(&self) -> i32 {
        *self.level.iter().max().unwrap()
    }

    fn update(&mut self, points_of_contact: &Vec<(i32, i32)>) {
        for &(x, y) in points_of_contact.iter() {
            self.level[x as usize] = y;
        }
    }
}


fn tetris_game(jet_pattern: &Vec<i32>, game_duration: u32) -> i32 {
    let mut game_elapsed = 0;
    let mut height: i32;
    let mut contact_level = ContactLevel::new();
    let mut jet_stream_iter = jet_pattern.iter().cycle();
    let mut existing_blocks: HashSet<(i32, i32)> = HashSet::new();
    'game: loop {
        for i_tetromino in 0..N_TETROMINOS {
            height = contact_level.max() + 4;
            let mut tetromino = spawn_tetromino(i_tetromino, height);
            loop  {
                let jet_stream = jet_stream_iter.next().unwrap();
                if !blocked_by_existing_blocks(&tetromino, &existing_blocks, jet_stream) {
                    tetromino.move_by_jet_stream(jet_stream);
                }
                if tetromino.collision_check(&contact_level.level) {
                    break;
                }
                tetromino.move_down();
            }
            let points_of_contact = tetromino.new_points_of_contact();
            contact_level.update(&points_of_contact);
            tetromino.all_blocks().iter().for_each(|&block| {existing_blocks.insert(block);});
            // println!("asfd");
            game_elapsed += 1;
            if game_elapsed == game_duration {
                break 'game;
            }
        }
    }
    contact_level.max()
}

fn spawn_tetromino(i_tetromino: usize, height: i32) -> Tetromino {
    match i_tetromino {
        0 => Tetromino::Hyphen((2, height)),
        1 => Tetromino::Plus((3, height)),
        2 => Tetromino::RightAngle((2, height)),
        3 => Tetromino::Pipe((2, height)),
        4 => Tetromino::Square((2, height)),
        _ => panic!("???")
    }
}

fn blocked_by_existing_blocks(tetromino: &Tetromino, existing_blocks: &HashSet<(i32, i32)>, jet_stream: &i32) -> bool {
    let all_blocks = tetromino.all_blocks();
    for &(x, y) in all_blocks.iter() {
        if existing_blocks.contains(&(x+jet_stream, y)) {
            return true;
        }
    }
    false
}


fn main() {
    let input = include_str!("../../inputs/day17.in");
    let jet_pattern = input
        .chars()
        .map(|c| match c {
            '>' => 1,
            '<' => -1,
            _ => panic!("???")
        })
        .collect::<Vec<i32>>();

    let tower_height = tetris_game(&jet_pattern, 2022);
    println!("Part 1: {}", tower_height);
}
