from dataclasses import dataclass
from typing import Dict, List
import copy

from reversi.board import *
from reversi.move import Move


@dataclass(frozen=True)
class Action:
    color: Square = Square.BLACK
    move: Move = Move.PASS


class BoardDir(Enum):
    UP_LEFT = (-1, -1)
    UP = (-1, 0)
    UP_RIGHT = (-1, 0)
    LEFT = (0, -1)
    RIGHT = (0, 1)
    DOWN_LEFT = (1, -1)
    DOWN = (1, 0)
    DOWN_RIGHT = (1, 1)


@dataclass(frozen=True)
class GameState:
    board: Board = None
    color: Square = Square.BLACK
    depth: int = 0

    def next_state(self, action: Action) -> 'GameState':
        if action.color != self.color:
            return None

        if action.move == Move.PASS:
            if self.is_pass():
                board = copy.deepcopy(self.board)
                return GameState(board, self._opponent_color(), self.depth + 1)
            else:
                return None

        info: Dict[BoardDir, int] = self._get_flip_info(action)
        if sum(info.values()) == 0:
            return None

        board: Board = copy.deepcopy(self.board)
        row, col = action.move.value
        board[row, col] = action.color
        for dir in BoardDir:
            ir = row + dir.value[0]
            ic = col + dir.value[1]
            for _ in range(info[dir]):
                board[ir, ic] = action.color
                ir = ir + dir.value[0]
                ic = ic + dir.value[1]

        return GameState(board, self._opponent_color(), self.depth + 1)

    def valid_actions(self) -> List[Action]:
        actions: List[Action] = []
        for r in range(Board.BOARD_SIZE):
            for c in range(Board.BOARD_SIZE):
                action = Action(color=self.color, move=Move.from_row_col(r, c))
                info: Dict[BoardDir, int] = self._get_flip_info(action)
                if sum(info.values()) > 0:
                    actions.append(action)

        if len(actions) == 0:
            actions.append(Action(color=self.color, move=Move.PASS))

        return actions

    def is_pass(self) -> bool:
        actions = self.valid_actions()
        return len(actions) == 1 and actions[0].move == Move.PASS

    def is_end(self) -> bool:
        if not self.is_pass():
            return False
        pass_action = Action(color=self.color, move=Move.PASS)
        next_state = self.next_state(pass_action)
        return next_state.is_pass()

    def black_count(self) -> int:
        return self.board.black_count()

    def white_count(self) -> int:
        return self.board.white_count()

    def black_squares(self) -> List[int]:
        return self.board.black_squares()

    def white_squares(self) -> List[int]:
        return self.board.white_squares()

    def to_string(self) -> str:
        lines = [f'[{self.depth}, {self.color}]',
                 '   a b c d e f g h ', '   ----------------']
        for r in range(Board.BOARD_SIZE):
            items = [f'{r + 1}|']
            for c in range(Board.BOARD_SIZE):
                s = self.board[r, c]
                items.append(
                    'b' if s == Square.BLACK else 'w' if s == Square.WHITE else '.')
            line = ' '.join(items)
            lines.append(line)
        return '\n'.join(lines)

    def equals_to(self, other: 'GameState') -> bool:
        if self.color != other.color or self.depth != other.depth:
            return False

        if self.board is None and other.board is None:
            return True

        if self.board is None or other.board is None:
            return False

        self_squares = list(chain.from_iterable(self.board.squares))
        other_squares = list(chain.from_iterable(other.board.squares))
        return self_squares == other_squares

    def _opponent_color(self) -> Square:
        if self.color == Square.EMPTY:
            raise Exception
        return Square.BLACK if self.color == Square.WHITE else Square.WHITE

    def _get_flip_info(self, action: Action) -> Dict[BoardDir, int]:
        info: Dict[BoardDir, int] = {
            BoardDir.UP_LEFT: 0,
            BoardDir.UP: 0,
            BoardDir.UP_RIGHT: 0,
            BoardDir.LEFT: 0,
            BoardDir.RIGHT: 0,
            BoardDir.DOWN_LEFT: 0,
            BoardDir.DOWN: 0,
            BoardDir.DOWN_RIGHT: 0,
        }

        if action.move == Move.PASS:
            return info

        row, col = action.move.value
        if not Board.is_valid_position(row, col):
            return info

        if self.board[row, col] != Square.EMPTY:
            return info

        for dir in BoardDir:
            ir: int = row + dir.value[0]
            ic: int = col + dir.value[1]

            opponent = Square.BLACK if action.color == Square.WHITE else Square.WHITE

            while Board.is_valid_position(ir, ic) and self.board[ir, ic] == opponent:
                ir = ir + dir.value[0]
                ic = ic + dir.value[1]

            if not Board.is_valid_position(ir, ic) or self.board[ir, ic] != self.color:
                continue

            dr: int = abs(ir - row)
            dc: int = abs(ic - col)
            distance: int = dr if dr >= dc else dc

            info[dir] = distance - 1

        return info
