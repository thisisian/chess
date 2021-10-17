

pub struct Move {
    from_to: (Square, Square),
    move_type: MoveType
}
type Offset = u64;

#[derive(Debug)]
enum MoveType {
    Quiet,
    Capture,
    CastleQueen,
    CastleKing,
    Promote(PromotionType)
    PromoteCapture(PromotionType)
}

#[derive(Debug))]
enum PromotionType {
    Queen,
    Rook,
    Bishop,
    Knight
}

// Direction and a distance
type Compass = (Square, Direction, u8);

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Equal,
}

// Convert (North, 8) to Movement Squares
fn direction_to_move((s1, direction, dist): Compass) -> Square {
    if (direction == Equal){
        return 0;
    }

    let x = match direction {
        N => 8,
        NE => 9,
        E => 1,
        SE => -7,
        S => -8,
        SW => -9,
        W => -1,
        Equal => 0
    };
    s + x * dist
}
