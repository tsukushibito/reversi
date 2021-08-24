from copy import deepcopy
from dataclasses import dataclass
from enum import Enum, auto
from typing import List, ClassVar


class Square(Enum):
    EMPTY = auto()
    BLACK = auto()
    WHITE = auto()


@dataclass(flozen=True)
class Action:
    row: int
    col: int
    is_pass: bool


@dataclass
class Board:
    BOARD_SIZE: ClassVar[int] = 8

    squares: List[List[Square]] = None

    def initial() -> 'Board':
        squares = [[Square.EMPTY] *
                   Board.BOARD_SIZE for i in range(Board.BOARD_SIZE)]

        init_pos = Board.BOARD_SIZE // 2
        squares[init_pos][init_pos] = Square.WHITE
        squares[init_pos + 1][init_pos + 1] = Square.WHITE
        squares[init_pos + 1][init_pos] = Square.BLACK
        squares[init_pos][init_pos + 1] = Square.BLACK

        return Board(squares)
