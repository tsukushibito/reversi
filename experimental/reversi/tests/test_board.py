import unittest
from reversi import Board, Square


class TestReversi(unittest.TestCase):

    def test_board_apply_action(self):
        board = Board()
        board2 = board.apply_action()
        board.squares[0][0] = Square.BLACK
        print(board)
