use crate::board::Square;

enum Direction {
    Left2Right,
    Top2Bottom,
    TopLeft2BottomRight,
    BottomLeft2TopRight,
}

#[derive(Clone, Copy)]
struct MobilityInfo {
    pub movables: u8,
    pub mobility_info: [(u8, u8); 8],
}

const INDEX_COUNT: usize = 3 ^ 8;

pub struct Indexer {
    mobilityTableForBlack: [MobilityInfo; INDEX_COUNT],
    mobilityTableForWhite: [MobilityInfo; INDEX_COUNT],
}

impl Indexer {
    pub fn new() -> Indexer {
        let info = [(0, 0); 8];
        let mut black = [MobilityInfo {
            movables: 0,
            mobility_info: info,
        }; INDEX_COUNT];
        let mut white = [MobilityInfo {
            movables: 0,
            mobility_info: info,
        }; INDEX_COUNT];

        for i in 0..INDEX_COUNT {
            let line = index2Line(i);

            for p in 0..8 {
                black[i] = MobilityInfo {
                    movables: 0,
                    mobility_info: info,
                };
                white[i] = MobilityInfo {
                    movables: 0,
                    mobility_info: info,
                };
            }
        }

        Indexer {
            mobilityTableForBlack: black,
            mobilityTableForWhite: white,
        }
    }
}

fn index2Line(index: usize) -> [Square; 8] {
    let mut line = [Square::Empty; 8];
    let mut i = index;

    for n in 0..8 {
        line[n] = match i % 3 {
            0 => Square::White,
            1 => Square::Empty,
            2 => Square::Black,
            _ => Square::Empty,
        };
        i = i / 3;
    }

    line
}
