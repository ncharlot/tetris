use rand::{thread_rng, Rng};


type Color = (u8, u8, u8);


#[derive(Clone)]
struct Tetromino {
    shape: [[u8; 4]; 4],
    color: Color,
}


const TETROMINO_STRAIGHT: Tetromino = Tetromino {
    shape: [
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
    ],
    color: (255, 0, 0),
};

const TETROMINO_straight: Tetromino = Tetromino {
    shape: [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    color: (0, 255, 0),
};


impl Tetromino {
    fn new_random() -> Tetromino {
        let mut rng = thread_rng();

        let mut new_tetromino: Tetromino = match rng.gen_range(0..2) {
            0 => TETROMINO_STRAIGHT.clone(),
            _ => TETROMINO_straight.clone(),
        };

        for _ in 0..rng.gen_range(0..4) {
            new_tetromino.rotate90(true);
        }

        new_tetromino
    }

    fn rotate90(&mut self, clockwise: bool) {
        let mut rotated_shape: [[u8; 4]; 4] = [[0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                if clockwise {
                    rotated_shape[i][j] = self.shape[3-j][i];
                } else {
                    rotated_shape[i][j] = self.shape[j][3-i];
                }
            }
        }

        self.shape = rotated_shape;
    }
}


fn main() {
    let mut new_tetromino: Tetromino = Tetromino::new_random();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate90() {
        let mut tetromino_straight = TETROMINO_STRAIGHT.clone();

        tetromino_straight.rotate90(true);
        assert_eq!(
            tetromino_straight.shape,
            [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ]
        );

        tetromino_straight.rotate90(false);
        assert_eq!(
            tetromino_straight.shape,
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ]
        );

        tetromino_straight.rotate90(true);
        tetromino_straight.rotate90(true);
        assert_eq!(
            tetromino_straight.shape,
            [
                [0, 0, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 1, 0],
            ]
        );

        tetromino_straight.rotate90(true);
        assert_eq!(
            tetromino_straight.shape,
            [
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
            ]
        );

        tetromino_straight.rotate90(true);
        assert_eq!(
            tetromino_straight.shape,
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ]
        );
    }
}