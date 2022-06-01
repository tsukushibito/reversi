from dataclasses import dataclass
from typing import List
from itertools import chain

from reversi import Player
from reversi import Action, GameState

from reversi_ai import Mcts, MctsTreeNode, DualNetwork


@dataclass
class SearchRecord:
    state: GameState
    actions: List[Action]
    scores: List[float]


class AiPlayer(Player):
    def __init__(self, name: str, dual_network: DualNetwork, evaluation_count: int, temperature: int = 0):
        self._name = name
        self._mcts = Mcts(dual_network=dual_network)
        self._tree_root = None
        self._temperature = temperature
        self._evaluation_count = evaluation_count
        self._records: List[SearchRecord] = []

    def take_action(self, state: GameState) -> Action:
        print(self._name)
        print(state.to_string())

        if self._tree_root is None or self._tree_root.children is None:
            self._tree_root = MctsTreeNode.new(state)
        else:
            temp = [
                c.children for c in self._tree_root.children if c.children is not None]
            nodes = chain.from_iterable(temp)

            new_root: MctsTreeNode = None
            for n in nodes:
                if n.state.equals_to(state):
                    new_root = n
                    break

            if new_root is None:
                new_root = MctsTreeNode.new(state)

            self._tree_root = new_root

        result = self._mcts.search(
            self._tree_root, self._evaluation_count, self._temperature)
        self._records.append(SearchRecord(
            state=state, actions=result.actions, scores=result.scores))
        return result.select_action()

    @property
    def records(self):
        return self._records
