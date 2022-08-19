import 'dart:async';
import 'dart:collection';

import 'package:app/features/game/application/game_application.dart';
import 'package:app/features/game/domain/reversi.dart' as r;
import 'package:hooks_riverpod/hooks_riverpod.dart';

class GameViewModelState {
  final UnmodifiableListView<int> squares;
  final String blackCount;
  final String whiteCount;
  final String message;
  final List<r.Position> movables;

  GameViewModelState({
    required this.squares,
    required this.blackCount,
    required this.whiteCount,
    required this.message,
    required this.movables,
  });

  factory GameViewModelState.init() {
    var board = r.Board.initial();
    return GameViewModelState(
        squares: board.squares,
        blackCount: '',
        whiteCount: '',
        message: '',
        movables: []);
  }
}

class GameViewModel extends StateNotifier<GameViewModelState> {
  GameViewModel(GameApplication game)
      : _game = game,
        super(GameViewModelState.init()) {
    state = _createState();
    _changeMessageSubscription =
        _game.changeMessage.listen((_) => state = _createState());
  }

  final GameApplication _game;
  late StreamSubscription _changeMessageSubscription;

  void receivePosition(r.Position position) => _game.move(position);
  void reset() => _game.reset();

  @override
  void dispose() {
    super.dispose();
    _changeMessageSubscription.cancel();
  }

  GameViewModelState _createState() {
    final board = _game.board;
    final UnmodifiableListView<int> squares = board.squares;
    final String blackCount = 'Black: ${board.blackCount()}';
    final String whiteCount = 'White: ${board.whiteCount()}';
    final String message = _createMessage();
    final List<r.Position> movables = board.getMovablePositions(board.turn());
    return GameViewModelState(
      squares: squares,
      blackCount: blackCount,
      whiteCount: whiteCount,
      message: message,
      movables: movables,
    );
  }

  String _createMessage() {
    final board = _game.board;
    if (board.isGameOver()) {
      if (board.blackCount() > board.whiteCount()) {
        return 'Black wins!';
      } else if (board.blackCount() < board.whiteCount()) {
        return 'White wins!';
      } else {
        return 'Draw';
      }
    } else {
      if (board.getMovablePositions(board.turn()).isEmpty) {
        return 'Pass';
      } else {
        return 'Turn: ${board.turn() == r.black ? 'Black' : 'White'}';
      }
    }
  }
}
