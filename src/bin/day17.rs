
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
    fn move_by_jet_stream(&mut self, jet_stream: i32) {
        match self {
            Tetromino::Hyphen((x, _)) => *x = (*x + jet_stream).max(0).min(3),
            Tetromino::Plus((x, _)) => *x = (*x + jet_stream).max(2).min(5),
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

    fn collision_check(&self, contact_level: &Vec<i32>) -> Some(i32) {
        match self {
            Tetromino::Hyphen((x, y)) => {
                let points_of_impact = vec![(*x, y-1), (*x-1, y), (*x+1, y)];
                check_collision(&points_of_impact, contact_level)

            },
            Tetromino::Plus((x, y)) => {
                let points_of_impact = vec![(*x, y-1), (*x-1, y), (*x+1, y)];
                check_collision(&points_of_impact, contact_level)
            },
            Tetromino::RightAngle((x, y)) =>
                {
                    let points_of_impact = vec![(*x, y-1), (*x+1, y-1), (*x+2, y-1)];
                    check_collision(&points_of_impact, contact_level)

                },
            Tetromino::Pipe((x, y)) => {
                let points_of_impact = vec![(*x, y-1)];
                check_collision(&points_of_impact, contact_level)
            }
            Tetromino::Square((x, y)) => {
                let points_of_impact = vec![(*x, y-1), (*x+1, y-1)];
                check_collision(&points_of_impact, contact_level)
            }
        }
    }
}

fn check_collision(points: &[(i32, i32)], contact_level: &Vec<i32>) -> Option<i32> {
    for &(x, y) in points.iter() {
        if y == contact_level[x as usize] {
            return Some(x);
        }
    }
    None
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

    fn update(&mut self, tetromino: &Tetromino, x: i32) {
        todo!("update contact level")
    }
}


fn tetris_game(jet_pattern: &Vec<i32>, game_duration: u32) -> i32 {
    let game_elapsed = 0;
    let mut height = 0;
    let mut contact_level = ContactLevel::new();
    while game_elapsed < game_duration + 1 {
        for i_tetromino in 0..N_TETROMINOS {
            height = contact_level.max() + 4;
            let mut tetromino = spawn_tetromino(i_tetromino, height);
            for jet_stream in jet_pattern.iter() {
                tetromino.move_by_jet_stream(*jet_stream);
                tetromino.move_down();
                if Some(x) = tetromino.collision_check(&contact_level.level) {
                    break;
                }
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

}