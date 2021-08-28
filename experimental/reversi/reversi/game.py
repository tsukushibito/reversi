from dataclasses import dataclass
from reversi.player import Player
from reversi.game_state import Action, GameState
from reversi.board import Board


@dataclass
class GameResult:
    depth: int
    black: int
    white: int
    black_win: bool
    white_win: bool
    draw: bool


class Game:
    def __init__(self, black_player: Player, white_player: Player):
        self._black_player: Player = black_player
        self._white_player: Player = white_player
        self.state: GameState = GameState(Board.initial())

    def run(self) -> GameResult:
        is_passed: bool = False
        while True:
            player: Player = \
                self._black_player if self.state.depth // 2 == 0 else self._white_player
            action: Action = player.take_action(self.state)
            if action == None:
                raise Exception('Player.take_action returned None object')

            next_state: GameState = self.state.next_state(action)
            if next_state is None:
                raise Exception('Invalid action was detected')

            self.state = next_state

            if is_passed and action.is_pass:
                break

            is_passed = action.is_pass

        black_count = self.state.black_count()
        white_count = self.state.white_count()
        return GameResult(
            depth=self.state.depth,
            black=black_count,
            white=white_count,
            black_win=black_count > white_count,
            white_win=white_count > black_count,
            draw=black_count == white_count)
