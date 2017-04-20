
#[derive(Debug, Clone, Copy, PartialEq,)]
pub struct ChessPosition {
    x: i8,
    y: i8,
}

pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    fn valid_position(pos: i8) -> bool {
        match pos {
            0...7 => true,
            _ => false,
        }
    }

    pub fn new(x: i8, y: i8) -> Result<ChessPosition, ()> {
        if !ChessPosition::valid_position(x) || !ChessPosition::valid_position(y) {
            return Err(());
        }
        Ok(ChessPosition { x: x, y: y })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Queen {
        Queen { position: position }
    }

    pub fn can_attack(self, other: &Queen) -> bool {
        if self.position.x == other.position.x {
            true
        } else if self.position.y == other.position.y {
            true
        }    else if self.check_diagonals(&other.position) {
            true
        } else {
            false
        }
    }

    fn check_diagonals(self, position: &ChessPosition) -> bool {
        let diff_x = (self.position.x - position.x).abs();
        let diff_y = (self.position.y - position.y).abs();

        diff_x == diff_y
    }
}