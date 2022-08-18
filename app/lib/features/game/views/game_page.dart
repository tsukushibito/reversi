import 'package:app/features/game/reversi.dart' as reversi;
import 'package:app/features/game/views/board.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class GamePage extends HookConsumerWidget {
  const GamePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var boardState = useState(reversi.Board.initial());
    var board = boardState.value;

    var isPass =
        !board.isGameOver() && board.getMovablePositions(board.turn()).isEmpty;

    if (isPass) {
      Future.delayed(const Duration(seconds: 3), () {
        var next = board.applyAction(
            reversi.Action(board.turn(), reversi.Position(0, 0), true));
        if (next != null) {
          boardState.value = next;
        }
      });
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Reversi'),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              Text('Black: ${board.blackCount()}'),
              Text('White: ${board.whiteCount()}'),
            ],
          ),
          Expanded(
            child: Center(
              child: Board(
                board: board,
                onTap: (row, col) {
                  debugPrint('r: $row, c: $col');
                  if (!isPass) _move(row, col, boardState);
                },
              ),
            ),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              ElevatedButton(
                onPressed: () {
                  boardState.value = reversi.Board.initial();
                },
                child: const Text('Reset'),
              )
            ],
          ),
        ],
      ),
    );
  }

  void _move(int row, int col, ValueNotifier<reversi.Board> boardState) {
    var board = boardState.value;
    if (board.isGameOver()) return;
    var action =
        reversi.Action(board.turn(), reversi.Position(row, col), false);
    var next = board.applyAction(action);
    if (next != null) {
      boardState.value = next;
    }
  }
}
