import 'package:app/features/game/models/reversi.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('Board.applyAction test', () {
    test('applyAction with invalid action returns null.', () {
      var board = Board.initial();

      var act = Action(black, Position(0, 0), false);
      var next = board.applyAction(act);
      expect(next, isNull);

      act = Action(black, Position(0, 0), true);
      next = board.applyAction(act);
      expect(next, isNull);
    });

    test('applyAction with valid action returns valid board.', () {
      var board = Board.initial();

      var act = Action(black, Position(2, 3), false);
      var next = board.applyAction(act);
      expect(next, isNotNull);
      expect(next?.squares[positionToIndex(Position(2, 3))], black);
      expect(next?.squares[positionToIndex(Position(3, 3))], black);
      expect(next?.squares[positionToIndex(Position(4, 3))], black);
      expect(next?.squares[positionToIndex(Position(3, 4))], black);
      expect(next?.squares[positionToIndex(Position(4, 4))], white);

      act = Action(white, Position(2, 2), false);
      next = next?.applyAction(act);
      expect(next, isNotNull);
      expect(next?.squares[positionToIndex(Position(2, 3))], black);
      expect(next?.squares[positionToIndex(Position(4, 3))], black);
      expect(next?.squares[positionToIndex(Position(3, 4))], black);
      expect(next?.squares[positionToIndex(Position(2, 2))], white);
      expect(next?.squares[positionToIndex(Position(3, 3))], white);
      expect(next?.squares[positionToIndex(Position(4, 4))], white);
    });
  });
}
