from dataclasses import dataclass
from typing import List

import tensorflow as tf

from reversi import GameState, Square


@dataclass
class GameTreeNode:
    """ゲーム木ノード

    Attributes:
        children(List[GameTreeNode]): 子ノードリスト
        state(GameState): ゲーム状態
        policy: 方策(このノードが選ばれる確率)
        weight: 累計価値
        evaluated_count: 試行回数
    """

    children: List['GameTreeNode'] = None
    state: GameState = None
    policy: float = 0.0
    weight: float = 0.0

    def evaluate(self) -> float:
        """このノードを評価

        Returns:
            float: 評価値
        """
        # ゲーム終了時
        if self.state.is_end():
            # 勝敗結果で価値を取得
            black_count: int = self.state.black_count()
            white_count: int = self.state.white_count()
            is_draw: bool = black_count == white_count
            wins: bool = black_count > white_count if self.state.color == Square.BLACK else white_count > black_count
            value: int = 0 if is_draw else \
                1 if wins else \
                -1

            # 累計価値と試行回数の更新
            self.w += value
            self.n += 1
            return value

        # 子ノードが存在しない時
        if not self.child_nodes:
            # ニューラルネットワークの推論で方策と価値を取得
            policies, value = predict(model, self.state)

            # 累計価値と試行回数の更新
            self.w += value
            self.n += 1

            # 子ノードの展開
            self.child_nodes = []
            for action, policy in zip(self.state.legal_actions(), policies):
                self.child_nodes.append(Node(self.state.next(action), policy))
            return value

        # 子ノードが存在する時
        else:
            # アーク評価値が最大の子ノードの評価で価値を取得
            value = -self.next_child_node().evaluate()

            # 累計価値と試行回数の更新
            self.w += value
            self.n += 1
            return value

    def expand():
        pass
