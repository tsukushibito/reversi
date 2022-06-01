from dataclasses import dataclass
from typing import List
from numpy import square
from reversi import Game, GameResult, Board, Square, Move
from reversi_ai import DualNetwork, AiPlayer, SearchRecord, TrainingData


@dataclass
class PlayData:
    pass


class SelfPlay:
    def __init__(self, dual_network: DualNetwork, evaluation_count: int, temperature: float) -> None:
        self._dual_network = dual_network
        self._player0 = AiPlayer(
            'player0', dual_network, evaluation_count, temperature)
        self._player1 = AiPlayer(
            'player1', dual_network, evaluation_count, temperature)

    def run(self) -> List[TrainingData]:
        game = Game(self._player0, self._player1)
        result = game.run()
        rec = []
        rec.extend(self._player0.records)
        rec.extend(self._player1.records)

        training_datas = self._make_training_datas(result, rec)
        return training_datas

    def _make_training_datas(self, result: GameResult, records: List[SearchRecord]) -> List[TrainingData]:
        datas = []
        for record in records:
            policies = [0] * DualNetwork.POLICY_COUNT
            for action, policy in zip(record.actions, record.scores):
                index = Board.BOARD_SIZE * action.move.value[0] + action.move.value[1] \
                    if action.move != Move.PASS \
                    else Board.BOARD_SIZE * Board.BOARD_SIZE
                policies[index] = policy
            black_squares = record.state.black_squares()
            white_squares = record.state.white_squares()
            if record.state.color == Square.BLACK:
                squares = black_squares
                opponent_squares = white_squares
                value = 0 if result.draw \
                    else 1 if result.black_win \
                    else -1
            else:
                squares = white_squares
                opponent_squares = black_squares
                value = 0 if result.draw \
                    else 1 if result.white_win \
                    else -1

            data = TrainingData(
                squares=squares, opponent_squares=opponent_squares, policies=policies, value=value)
            datas.append(data)

        return datas
