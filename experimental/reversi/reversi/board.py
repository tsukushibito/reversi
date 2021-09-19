from copy import deepcopy
from dataclasses import dataclass
from enum import Enum, auto
from typing import List, ClassVar
from itertools import chain


class Square(Enum):
    EMPTY = auto()
    BLACK = auto()
    WHITE = auto()


@dataclass
class Board:
    BOARD_SIZE: ClassVar[int] = 8

    squares: List[List[Square]] = None

    def __getitem__(self, pos):
        return self.squares[pos[0]][pos[1]]

    def __setitem__(self, pos, s):
        self.squares[pos[0]][pos[1]] = s

    @classmethod
    def initial(cls) -> 'Board':
        squares = [[Square.EMPTY] *
                   Board.BOARD_SIZE for i in range(Board.BOARD_SIZE)]

        init_pos = (Board.BOARD_SIZE - 1) // 2
        squares[init_pos][init_pos] = Square.WHITE
        squares[init_pos + 1][init_pos + 1] = Square.WHITE
        squares[init_pos + 1][init_pos] = Square.BLACK
        squares[init_pos][init_pos + 1] = Square.BLACK

        return Board(squares)

    def is_valid_position(row: int, col: int) -> bool:
        return row >= 0 and row < Board.BOARD_SIZE \
            and col >= 0 and col < Board.BOARD_SIZE

    def count_of(self, square: Square) -> int:
        return len([s for s in chain.from_iterable(self.squares) if s == square])

    def black_count(self) -> int:
        return self.count_of(Square.BLACK)

    def white_count(self) -> int:
        return self.count_of(Square.WHITE)

    def squares_of(self, square: Square) -> List[int]:
        return [1 if s == square else 0
                for s in chain.from_iterable(self.squares)]

    def black_squares(self) -> List[int]:
        return self.squares_of(Square.BLACK)

    def white_squares(self) -> List[int]:
        return self.squares_of(Square.WHITE)
