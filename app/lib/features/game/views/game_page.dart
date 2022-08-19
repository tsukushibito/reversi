import 'package:app/features/game/models/reversi.dart' as r;
import 'package:app/features/game/view_models/game_view_model.dart';
import 'package:app/features/game/views/board.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class GamePage extends HookConsumerWidget {
  const GamePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    var viewModelState = useState(GameViewModel.init());
    var viewModel = viewModelState.value;
    viewModel.updateEvent.listen((event) {
      viewModelState.value = event;
    });

    return Scaffold(
      appBar: AppBar(
        title: const Text('Reversi'),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          Center(
            child: Text(viewModel.message),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              Text(viewModel.blackCount),
              Text(viewModel.whiteCount),
            ],
          ),
          Expanded(
            child: Center(
              child: Board(
                board: viewModel.squares,
                onTap: (row, col) {
                  debugPrint('r: $row, c: $col');
                  viewModel.receivePosition(r.Position(row, col));
                },
              ),
            ),
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              ElevatedButton(
                onPressed: () {
                  viewModel.reset();
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
