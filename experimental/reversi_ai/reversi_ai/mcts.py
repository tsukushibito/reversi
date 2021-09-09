from dataclasses import dataclass
from typing import ClassVar, Dict, List
from math import sqrt

import numpy as np

from reversi import Square, GameState
from reversi_ai import DualNetwork


@dataclass
class MctsTreeNode:
    """ゲーム木ノード

    Attributes:
        children(List[GameTreeNode]): 子ノードリスト
        state(GameState): ゲーム状態
        policy: 方策(このノードが選ばれる確率)
        weight: 累計価値
        evaluated_count: 試行回数
    """
    C_PUCT: ClassVar[float] = 1.0

    state: GameState = None
    children: List['MctsTreeNode'] = None
    policy: float = 0.0
    weight: float = 0.0
    evaluated_count: int = 0


class Mcts:
    """モンテカルロ木探索クラス
    Attributes:
        _dual_network: デュアルネットワーク
        _node_data_table: ノードデータテーブル ノードをキーとして、そのノードに対応するデータを値とします
    """
    C_PUCT: ClassVar[float] = 1.0

    def __init__(self, dual_network: DualNetwork):
        """[summary]

        Args:
            dual_network (DualNetwork): [description]
        """
        self._dual_network: DualNetwork = dual_network

    def search(self, node: MctsTreeNode) -> MctsTreeNode:
        self._evaluate_node(node)
        return node.children[0]  # 仮実装

    def _evaluate_node(self, node: MctsTreeNode) -> float:
        """ノードを評価

        Args:
            node (GameTreeNode): ノード

        Returns:
            float: 評価値
        """

        value: float = 0.0

        # ゲーム終了時
        if node.state.is_end():
            # 勝敗結果で価値を取得
            black_count: int = node.state.black_count()
            white_count: int = node.state.white_count()
            is_draw: bool = black_count == white_count
            wins: bool = black_count > white_count \
                if node.state.color == Square.BLACK \
                else white_count > black_count
            value: int = 0 if is_draw else \
                1 if wins else \
                -1

        # 子ノードが存在しない時
        elif node.children is None:
            # ニューラルネットワークの推論で方策と価値を取得
            policies, value = self._dual_network.predict(node.state)

            # 子ノードの展開
            node.children = list(
                map(lambda action, policy:
                    MctsTreeNode(state=node.state.next_state(
                        action), policy=policy),
                    node.state.valid_actions(), policies))

        # 子ノードが存在する時
        else:
            # アーク評価値が最大の子ノードの評価で価値を取得
            next_node: MctsTreeNode = self._select_next_node(node.children)
            # 相手番の評価値を得るので-1を掛ける
            value = -self._evaluate_node(next_node)

        # 累計価値と試行回数の更新
        node.weight += value
        node.evaluated_count += 1

        return value

    def _select_next_node(self, nodes: List[MctsTreeNode]) -> MctsTreeNode:
        # アーク評価値の計算
        t = sum(map(lambda n: n.evaluated_count, nodes))
        pucb_values = list(map(lambda n:
                               -n.weight / n.evaluated_count if n.evaluated_count != 0 else 0.0 +
                               Mcts.C_PUCT * n.policy *
                               sqrt(t) / (1 + n.evaluated_count),
                               nodes))

        # アーク評価値が最大の子ノードを返す
        return nodes[np.argmax(pucb_values)]
