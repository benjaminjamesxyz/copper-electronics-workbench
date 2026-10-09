import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pcb_design_calculator/diagrams/catalog.dart';
import 'package:pcb_design_calculator/diagrams/panel.dart';
import 'package:pcb_design_calculator/diagrams/primitives.dart';
import 'package:pcb_design_calculator/models.dart';

void main() {
  final catalog = <String, Calculator>{
    for (final file in Directory('../backend/catalog').listSync().whereType<File>().where((f) => f.path.endsWith('.json')))
      for (final entry in jsonDecode(file.readAsStringSync()) as List)
        (entry as Map<String, dynamic>)['id'] as String: Calculator(entry),
  };

  test('every selectable model has an illustration with valid quantity references', () {
    expect(calculatorDiagrams.keys.toSet(), catalog.keys.toSet());
    for (final entry in calculatorDiagrams.entries) {
      final calculator = catalog[entry.key]!;
      final quantities = {for (final q in calculator.inputs) q.key, for (final q in calculator.outputs) q.key};
      for (final mark in entry.value.marks.whereType<Label>()) {
        if (mark.quantity != null) {
          expect(quantities, contains(mark.quantity), reason: '${entry.key}: ${mark.text} would describe an unrelated quantity');
        }
      }
    }
  });

  testWidgets('narrow diagrams keep readable text, scroll to hidden geometry and expose a text alternative', (tester) async {
    tester.view.physicalSize = const Size(480, 1000);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final calculator = catalog['dc.ohm_current']!;
    final diagram = calculatorDiagrams[calculator.id]!;
    await tester.pumpWidget(MaterialApp(home: MediaQuery(
      data: const MediaQueryData(size: Size(480, 1000), textScaler: TextScaler.linear(2)),
      child: Scaffold(body: SingleChildScrollView(child: DiagramPanel(calculator: calculator, diagram: diagram))),
    )));
    await tester.pumpAndSettle();
    expect(find.bySemanticsLabel(diagram.title), findsWidgets);
    expect(find.text(diagram.caption), findsOneWidget);
    final canvas = find.byKey(ValueKey('diagram-canvas-${calculator.id}'));
    expect(tester.getSize(canvas), const Size(1400, 560));
    final horizontal = find.byWidgetPredicate((w) => w is SingleChildScrollView && w.scrollDirection == Axis.horizontal);
    final controller = tester.widget<SingleChildScrollView>(horizontal).controller!;
    await tester.drag(horizontal, const Offset(-300, 0));
    await tester.pumpAndSettle();
    expect(controller.offset, greaterThan(200), reason: 'Narrow windows must let users reach the rest of the schematic');
    expect(tester.takeException(), isNull);
    await tester.pumpWidget(const SizedBox.shrink());
  }, semanticsEnabled: true);
}
