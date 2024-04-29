fn main() {
    println!("Hello, world!");
}
pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: usize,
    roll: usize,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            frames: Vec::new(),
            current_frame: 0,
            roll: 0,
        }
    }

    pub fn roll(&mut self, pins: i32) {
        if self.roll == 0 {
            let frame = Frame { rolls: Vec::new() };
            self.frames.push(frame);
        }

        self.frames[self.current_frame].rolls.push(pins);
        if (self.roll == 1 || pins == 10) && self.current_frame < 9 {
            self.current_frame += 1;
            self.roll = 0;
        } else {
            self.roll += 1;
        }
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        for (i, frame) in self.frames.iter().enumerate() {
            if frame.rolls.first().unwrap() == &10 {
                score += self.sum_frame(frame) + self.sum_next_two_rolls(i);
            } else if self.sum_frame(frame) == 10 {
                score += self.sum_frame(frame) + self.frames[i + 1].rolls.first().unwrap();
            } else {
                score += self.sum_frame(frame);
            }
        }
        score
    }

    fn sum_next_two_rolls(&self, frame_index: usize) -> i32 {
        let rolls: i32 = self.frames[frame_index + 1..]
            .iter()
            .flat_map(|f| f.rolls.iter())
            .take(2)
            .sum();
        rolls
    }

    fn sum_frame(&self, frame: &Frame) -> i32 {
        frame.rolls.iter().sum()
    }
}

#[derive(Debug, PartialEq)]
struct Frame {
    rolls: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_single_pin() {
        let mut game = BowlingGame::new();
        game.roll(1);
        assert_eq!(game.score(), 1);
    }

    #[test]
    fn test_two_rolls_frame_total() {
        let mut game = BowlingGame::new();
        game.roll(4);
        game.roll(5);
        let expected = vec![Frame { rolls: vec![4, 5] }];
        assert_eq!(game.frames, expected);
        assert_eq!(game.score(), 9);
    }

    #[test]
    fn test_two_frames_total() {
        let mut game = BowlingGame::new();
        game.roll(4);
        game.roll(5);
        game.roll(3);
        game.roll(6);
        let expected = vec![Frame { rolls: vec![4, 5] }, Frame { rolls: vec![3, 6] }];
        assert_eq!(game.frames, expected);
        assert_eq!(game.score(), 18);
    }

    #[test]
    fn test_spare() {
        let mut game = BowlingGame::new();
        game.roll(4);
        game.roll(6);
        game.roll(3);
        game.roll(6);
        let expected = vec![Frame { rolls: vec![4, 6] }, Frame { rolls: vec![3, 6] }];
        assert_eq!(game.frames, expected);
        assert_eq!(game.score(), 22);
    }

    #[test]
    fn test_strike() {
        let mut game = BowlingGame::new();
        game.roll(10);
        game.roll(3);
        game.roll(6);
        let expected = vec![Frame { rolls: vec![10] }, Frame { rolls: vec![3, 6] }];
        assert_eq!(game.frames, expected);
        assert_eq!(game.score(), 28);
    }

    #[test]
    fn test_perfect_game() {
        let mut game = BowlingGame::new();
        for _ in 0..12 {
            game.roll(10);
        }
        let mut expected = Vec::new();
        for _ in 0..9 {
            expected.push(Frame { rolls: vec![10] });
        }
        expected.push(Frame {
            rolls: vec![10, 10, 10],
        });
        assert_eq!(game.frames, expected);
        assert_eq!(game.score(), 300);
    }

    #[test]
    fn test_almost_perfect_game() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            game.roll(10);
        }
        game.roll(9);
        game.roll(1);

        let mut expected = Vec::new();
        for _ in 0..9 {
            expected.push(Frame { rolls: vec![10] });
        }
        expected.push(Frame {
            rolls: vec![10, 9, 1],
        });

        assert_eq!(game.frames, expected);
        assert_eq!(game.score(), 289);
    }

    #[test]
    fn test_game_279() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            game.roll(10);
        }
        game.roll(9);
        game.roll(1);
        game.roll(10);

        let mut expected = Vec::new();
        for _ in 0..9 {
            expected.push(Frame { rolls: vec![10] });
        }
        expected.push(Frame {
            rolls: vec![9, 1, 10],
        });

        assert_eq!(game.frames, expected);
        assert_eq!(game.score(), 279);
    }
}
