use crate::board::Square;
use crate::board::BOARD_SIZE;

enum Direction {
    Left2Right,
    Top2Bottom,
    TopLeft2BottomRight,
    BottomLeft2TopRight,
}

#[derive(Clone, Copy, Default)]
struct FlipInfo {
    pub lower: u8,
    pub higher: u8,
}

#[derive(Clone, Copy, Default)]
struct MobilityInfo {
    pub movables: u8,
    pub flip_infos: [FlipInfo; BOARD_SIZE],
}

const INDEX_COUNT: usize = 3_u32.pow(BOARD_SIZE as u32) as usize;

pub struct Indexer {
    mobility_table_for_black: [MobilityInfo; INDEX_COUNT],
    mobility_table_for_white: [MobilityInfo; INDEX_COUNT],
}

impl Indexer {
    pub fn new() -> Indexer {
        Indexer {
            mobility_table_for_black: [MobilityInfo::default(); INDEX_COUNT],
            mobility_table_for_white: [MobilityInfo::default(); INDEX_COUNT],
        }
    }
}

fn index_to_line(index: usize) -> [Square; BOARD_SIZE] {
    let mut line = [Square::Empty; BOARD_SIZE];
    let mut i = index;

    for n in 0..BOARD_SIZE {
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

fn line_to_index(line: &[Square; BOARD_SIZE]) -> usize {
    let mut index = 0;

    for n in 0..BOARD_SIZE {
        let num = line[n] as u32;
        index += 3_u32.pow(n as u32) * num;
    }

    index as usize
}

fn create_mobility_table(color: Square) -> [MobilityInfo; INDEX_COUNT] {
    if matches!(color, Square::Empty) {
        panic!();
    }

    let opponent = match color {
        Square::White => Square::Black,
        Square::Black => Square::White,
        _ => panic!(),
    };

    let mut table = [MobilityInfo::default(); INDEX_COUNT];

    for i in 0..INDEX_COUNT {
        let info = &mut table[i];
        let line = index_to_line(i);

        for pos in 0..BOARD_SIZE as i32 {
            if line[pos as usize] != Square::Empty {
                continue;
            }

            let flip_info = &mut info.flip_infos[pos as usize];

            // lower方向にひっくり返せる石があるか探索
            let mut i_it = pos - 1;
            while i_it >= 0 && line[i_it as usize] == opponent {
                i_it -= 1;
            }

            if i_it >= 0 && pos - i_it > 1 && line[i_it as usize] == color {
                flip_info.lower = (pos - i_it - 1) as u8;
            }

            // higher方向にひっくり返せる石があるか探索
            i_it = pos + 1;
            while i_it < BOARD_SIZE as i32 && line[i_it as usize] == opponent {
                i_it += 1;
            }

            if i_it < BOARD_SIZE as i32 && i_it - pos > 1 && line[i_it as usize] == color {
                flip_info.higher = (i_it - pos - 1) as u8;
            }

            info.movables += flip_info.lower + flip_info.higher;
        }
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_index() {
        let line0 = [Square::White; BOARD_SIZE];
        let index0 = line_to_index(&line0);
        assert_eq!(0, index0);

        let line1 = [Square::Empty; BOARD_SIZE];
        let index1 = line_to_index(&line1);
        let mut expect1 = 0;
        for n in 0..BOARD_SIZE {
            expect1 += 3_u32.pow(n as u32);
        }
        assert_eq!(expect1, index1 as u32);

        let line2 = [Square::Black; BOARD_SIZE];
        let index2 = line_to_index(&line2);
        let mut expect2 = 0;
        for n in 0..BOARD_SIZE {
            expect2 += 2 * 3_u32.pow(n as u32);
        }
        assert_eq!(expect2, index2 as u32);
    }

    #[test]
    fn test_index_to_line() {
        let line = index_to_line(0);
        for s in line {
            assert!(matches!(Square::Empty, s));
        }

        let line1 = [
            Square::White,
            Square::Black,
            Square::Empty,
            Square::White,
            Square::Black,
            Square::Empty,
            Square::White,
            Square::Black,
        ];
        let index = line_to_index(&line1);
        let line2 = index_to_line(index);

        for (s1, s2) in line1.iter().zip(line2.iter()) {
            assert!(matches!(s1, s2));
        }
    }

    #[test]
    fn test_create_mobility_table() {
        let table = create_mobility_table(Square::Black);

        for info in table {}
    }
}
