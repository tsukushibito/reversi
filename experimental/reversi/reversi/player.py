from abc import ABC, abstractmethod
import random

from reversi.game_state import Action, GameState


class Player(ABC):
    @abstractmethod
    def take_action(self, state: GameState) -> Action:
        pass


class RandomPlayer(Player):
    def take_action(self, state: GameState) -> Action:
        actions = state.valid_actions()
        action = random.choice(actions)
        return action


class TerminalPlayer(Player):
    def take_action(self, state: GameState) -> Action:
        print(state.to_string())
        actions = state.valid_actions()
        if len(actions) == 1 and actions[0].is_pass:
            print('Pass!')
            return Action(color=state.color, is_pass=True)

        print('Please input position')
        action: Action = None
        while True:
            try:
                pos = input('>>')
                temp_col = pos[0]
                col = ord(temp_col) - ord('A')
                temp_row = pos[1]
                row = ord(temp_row) - ord('1')
                iter = filter(lambda a: a.row == row and a.col == col, actions)
                action = next(iter)
                break
            except:
                print('Invalid position')

        return action
