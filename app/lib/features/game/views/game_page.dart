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
                  _move(row, col, boardState);
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
      // パスかどうかチェック
      if (next.getMovablePositions(next.turn()).isEmpty) {
        // パスの場合は自動で進める
        next = next.applyAction(reversi.Action(
          next.turn(),
          reversi.Position(0, 0),
          true,
        ));
        if (next != null) {
          boardState.value = next;
        }
      }
    }
  }
}
