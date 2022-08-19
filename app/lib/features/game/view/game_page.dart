import 'package:app/features/game/domain/reversi.dart' as r;
import 'package:app/features/game/view_model/game_view_model.dart';
import 'package:app/features/game/view/board.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final _gameViewModelProvider =
    StateNotifierProvider<GameViewModel, GameViewModelState>(
        (ref) => GameViewModel(r.Board.initial()));

class GamePage extends HookConsumerWidget {
  const GamePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var state = ref.watch(_gameViewModelProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Reversi'),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          Center(
            child: Text(state.message),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              Text(state.blackCount),
              Text(state.whiteCount),
            ],
          ),
          Expanded(
            child: Center(
              child: Board(
                board: state.squares,
                onTap: (row, col) {
                  debugPrint('r: $row, c: $col');
                  ref
                      .read(_gameViewModelProvider.notifier)
                      .receivePosition(r.Position(row, col));
                },
              ),
            ),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              ElevatedButton(
                onPressed: () {
                  ref.read(_gameViewModelProvider.notifier).reset();
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
