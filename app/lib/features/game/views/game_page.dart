import 'package:app/features/game/reversi.dart' as reversi;
import 'package:app/features/game/views/board.dart';
import 'package:flutter/material.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class GamePage extends HookConsumerWidget {
  const GamePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var board = useState(reversi.Board.initial());
    return Scaffold(
      appBar: AppBar(
        title: const Text('Reversi'),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          Expanded(
            child: Center(
              child: Board(
                board: board.value,
                onTap: (row, col) {
                  debugPrint('r: $row, c: $col');
                  var b = board.value;
                  var action = reversi.Action(
                      b.turn(), reversi.Position(row, col), false);
                  var next = b.applyAction(action);
                  if (next != null) {
                    board.value = next;
                    // パスかどうかチェック
                    if (next.getMovablePositions(next.turn()).isEmpty) {
                      // パパの場合は自動で進める
                      next = next.applyAction(reversi.Action(
                          next.turn(), reversi.Position(0, 0), true));
                      if (next != null) {
                        board.value = next;
                      }
                    }
                  }
                },
              ),
            ),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              ElevatedButton(
                onPressed: () {
                  board.value = reversi.Board.initial();
                },
                child: const Text('Reset'),
              )
            ],
          ),
        ],
      ),
    );
  }
}
