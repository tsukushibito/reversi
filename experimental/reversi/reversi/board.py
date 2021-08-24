import copy
from enum import Enum, auto
from typing import List


class Square(Enum):
    EMPTY = auto()
    BLACK = auto()
    WHITE = auto()


class Board:
    BOARD_SIZE: int = 8

    squares: List[List[Square]]

    def __init__(self, squares: List[List[Square]] = None):
        if squares != None:
            self.squares = squares
        else:
            self.squares = [[Square.EMPTY] *
                            Board.BOARD_SIZE for i in range(Board.BOARD_SIZE)]

            init_pos = Board.BOARD_SIZE // 2
            self.squares[init_pos][init_pos] = Square.WHITE
            self.squares[init_pos + 1][init_pos + 1] = Square.WHITE
            self.squares[init_pos + 1][init_pos] = Square.BLACK
            self.squares[init_pos][init_pos + 1] = Square.BLACK

    def apply_action(self):
        return Board(copy.deepcopy(self.squares))
