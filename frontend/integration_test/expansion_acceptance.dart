import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:pcb_design_calculator/backend_client.dart';
import 'package:pcb_design_calculator/diagrams/catalog.dart';
import 'package:pcb_design_calculator/diagrams/primitives.dart';
import 'package:pcb_design_calculator/main.dart';
import 'package:pcb_design_calculator/models.dart';

// Temporary native acceptance capture; removed after the recorded delivery gate.
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets('all models remain selectable and every new illustrated example computes', (tester) async {
    IntegrationTestWidgetsFlutterBinding.instance.testTextInput.register();
    const lanes = {'sensors', 'opto', 'oscillators', 'rf', 'batteries', 'protection', 'converters', 'control', 'emc', 'cables', 'precision', 'logic', 'motors', 'passives', 'cores', 'measurement'};
    final backend = await BackendClient.start();
    final response = await backend.request({'command': 'catalog'});
    final catalog = (response['calculations'] as List).map((c) => Calculator(c as Map<String, dynamic>)).toList();
    await backend.close();
    final captures = Directory('/tmp/copper-expansion')..createSync(recursive: true);
    final issues = <String>[];
    final visited = <String>[];
    final computed = <String>[];
    final metadata = <Map<String, dynamic>>[];
    await tester.pumpWidget(const CopperApp());
    Future<void> waitFor(Finder finder) async {
      final deadline = DateTime.now().add(const Duration(seconds: 30));
      while (finder.evaluate().isEmpty && DateTime.now().isBefore(deadline)) {
        await tester.pump(const Duration(milliseconds: 100));
      }
      expect(finder, findsWidgets);
    }
    await waitFor(find.byKey(const ValueKey('catalog-search')));
    for (final calculator in catalog) {
      await tester.enterText(find.byKey(const ValueKey('catalog-search')), calculator.id);
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(ValueKey('select-${calculator.id}')));
      await tester.pumpAndSettle();
      expect(find.text(calculator.title), findsWidgets);
      final heading = '${calculator.category.toUpperCase()} / ${calculator.level.toUpperCase()}';
      expect(find.text(heading), findsOneWidget);
      final observedInputs = <Map<String, String>>[];
      for (final input in calculator.inputs) {
        final field = find.descendant(of: find.byKey(ValueKey('input-${input.key}')), matching: find.byType(TextField));
        final decoration = tester.widget<TextField>(field).decoration!;
        expect(decoration.labelText, input.label);
        expect(decoration.suffixText, input.unit);
        observedInputs.add({'key': input.key, 'label': decoration.labelText!, 'unit': decoration.suffixText!});
      }
      final canvas = find.byKey(ValueKey('diagram-canvas-${calculator.id}'));
      expect(canvas, findsOneWidget, reason: calculator.id);
      await tester.ensureVisible(canvas);
      await tester.pumpAndSettle();
      expect(tester.takeException(), isNull, reason: calculator.id);
      visited.add(calculator.id);
      if (lanes.contains(calculator.id.split('.').first)) {
        final diagram = calculatorDiagrams[calculator.id]!;
        final labels = diagram.marks.whereType<Label>().toList();
        final rectangles = <Rect>[];
        for (final label in labels) {
          final text = TextPainter(text: TextSpan(text: label.text, style: TextStyle(fontSize: label.size, fontWeight: FontWeight.w500)), textDirection: TextDirection.ltr)..layout();
          final rect = Offset(label.x, label.y) & text.size;
          rectangles.add(rect);
          if (rect.left < 0 || rect.top < 0 || rect.right > 700 || rect.bottom > 280) {
            issues.add('${calculator.id}: clipped label ${label.text}: $rect');
          }
          text.dispose();
        }
        for (var a = 0; a < labels.length; a++) {
          for (var b = a + 1; b < labels.length; b++) {
            if (rectangles[a].overlaps(rectangles[b])) {
              issues.add('${calculator.id}: overlapping labels ${labels[a].text} / ${labels[b].text}');
            }
          }
        }
        final boundary = tester.renderObject<RenderRepaintBoundary>(canvas);
        await tester.runAsync(() async {
          final image = await boundary.toImage(pixelRatio: 2);
          final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
          await File('${captures.path}/${calculator.id}.png').writeAsBytes(bytes!.buffer.asUint8List());
          image.dispose();
        });
      }
      final example = find.byKey(const ValueKey('load-example'));
      await tester.ensureVisible(example);
      await tester.pumpAndSettle();
      await tester.tap(example);
      await tester.pumpAndSettle();
      final calculate = find.byKey(const ValueKey('calculate'));
      await tester.ensureVisible(calculate);
      await tester.pumpAndSettle();
      await tester.tap(calculate);
      await waitFor(find.byKey(ValueKey('result-${calculator.outputs.first.key}')));
      final observedOutputs = <Map<String, String>>[];
      for (final output in calculator.outputs) {
        final rendered = tester.widget<SelectableText>(find.byKey(ValueKey('result-${output.key}'))).data!;
        expect(find.text(output.label), findsWidgets);
        expect(rendered.endsWith(' ${output.unit}'), isTrue);
        observedOutputs.add({'key': output.key, 'label': output.label, 'rendered': rendered});
        final actual = double.parse(rendered.split(' ').first);
        final expected = calculator.exampleExpected[output.key]!;
        expect(actual, closeTo(expected, expected.abs() * 1e-5 + 1e-300), reason: '${calculator.id}.${output.key}');
      }
      computed.add(calculator.id);
      metadata.add({'id': calculator.id, 'title': calculator.title, 'heading': heading, 'inputs': observedInputs, 'outputs': observedOutputs});
      expect(tester.takeException(), isNull, reason: calculator.id);
    }
    final newComputed = computed.where((id) => lanes.contains(id.split('.').first)).toList();
    File('/tmp/copper-expansion-acceptance.json').writeAsStringSync(const JsonEncoder.withIndent('  ').convert({'visited': visited, 'computed': computed, 'new_computed': newComputed, 'metadata': metadata, 'layout_issues': issues}));
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpAndSettle();
    expect(visited.length, 228);
    expect(computed.length, 228);
    expect(newComputed.length, 100);
    expect(issues, isEmpty);
  }, timeout: const Timeout(Duration(minutes: 20)));
  testWidgets('native filters recover and keyboard and process errors remain usable', (tester) async {
    IntegrationTestWidgetsFlutterBinding.instance.testTextInput.register();
    final backend = await BackendClient.start();
    final response = await backend.request({'command': 'catalog'});
    final catalog = (response['calculations'] as List).map((c) => Calculator(c as Map<String, dynamic>)).toList();
    await backend.close();
    await tester.pumpWidget(const CopperApp());
    Future<void> waitFor(Finder finder) async {
      final deadline = DateTime.now().add(const Duration(seconds: 30));
      while (finder.evaluate().isEmpty && DateTime.now().isBefore(deadline)) {
        await tester.pump(const Duration(milliseconds: 100));
      }
      expect(finder, findsWidgets);
    }
    Future<void> tapVisible(Finder finder) async {
      await tester.ensureVisible(finder);
      await tester.pumpAndSettle();
      await tester.tap(finder);
      await tester.pumpAndSettle();
    }
    await waitFor(find.byKey(const ValueKey('catalog-search')));
    await tester.enterText(find.byKey(const ValueKey('catalog-search')), '');
    await tester.pumpAndSettle();
    Future<void> chooseFilter(String current, String target) async {
      await tester.tap(find.text(current).last);
      await tester.pumpAndSettle();
      if (find.text(target).evaluate().isEmpty) {
        await tester.scrollUntilVisible(find.text(target), target.startsWith('All ') ? -200 : 200, scrollable: find.byType(Scrollable).last);
      }
      await tester.ensureVisible(find.text(target).last);
      await tester.pumpAndSettle();
      await tester.tap(find.text(target).last);
      await tester.pumpAndSettle();
    }
    final category = catalog.firstWhere((c) => c.id.startsWith('batteries.') && c.level == 'expert').category;
    await chooseFilter('All domains', category);
    await chooseFilter('All levels', 'expert');
    final filteredIds = catalog.where((c) => c.category == category && c.level == 'expert').map((c) => c.id).toList();
    expect(find.text('${filteredIds.length} matching calculations'), findsOneWidget);
    for (final id in filteredIds) {
      expect(find.byKey(ValueKey('select-$id')), findsOneWidget);
    }
    await chooseFilter(category, 'All domains');
    await chooseFilter('expert', 'All levels');
    final recoveryModel = catalog.firstWhere((c) => c.id == 'cables.twin_lead_geometry');
    await tester.enterText(find.byKey(const ValueKey('catalog-search')), recoveryModel.id);
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(ValueKey('select-${recoveryModel.id}')));
    await tester.pumpAndSettle();
    final spacing = find.byKey(const ValueKey('input-center_spacing'));
    final calculate = find.byKey(const ValueKey('calculate'));
    final loadExample = find.byKey(const ValueKey('load-example'));
    final result = find.byKey(ValueKey('result-${recoveryModel.outputs.first.key}'));
    final error = find.byKey(const ValueKey('calculation-error'));
    await tapVisible(loadExample);
    await tester.ensureVisible(spacing);
    await tester.pumpAndSettle();
    await tester.showKeyboard(spacing);
    await tester.testTextInput.receiveAction(TextInputAction.done);
    await waitFor(result);
    await tester.enterText(spacing, '0.001');
    await tester.pumpAndSettle();
    expect(result, findsNothing);
    await tapVisible(calculate);
    await waitFor(error);
    expect(result, findsNothing);
    final constraintError = tester.widget<SelectableText>(find.descendant(of: error, matching: find.byType(SelectableText))).data!;
    await tapVisible(loadExample);
    await tapVisible(calculate);
    await waitFor(result);
    expect(error, findsNothing);
    final recovered = tester.widget<SelectableText>(result).data!;
    expect(double.parse(recovered.split(' ').first), closeTo(recoveryModel.exampleExpected[recoveryModel.outputs.first.key]!, recoveryModel.exampleExpected[recoveryModel.outputs.first.key]!.abs() * 1e-5));
    final child = await Process.run('pgrep', ['-P', '$pid', '-x', 'pcb-calculator']);
    expect(child.exitCode, 0);
    final childIds = (child.stdout as String).trim().split('\n').map(int.parse).toList();
    expect(childIds.length, 1);
    expect(Process.killPid(childIds.single), isTrue);
    await tester.pump(const Duration(milliseconds: 100));
    await tapVisible(calculate);
    await waitFor(error);
    expect(result, findsNothing);
    final disconnectError = tester.widget<SelectableText>(find.descendant(of: error, matching: find.byType(SelectableText))).data!;
    File('/tmp/copper-expansion-recovery.json').writeAsStringSync(const JsonEncoder.withIndent('  ').convert({'model_id': recoveryModel.id, 'category_expert_filter_ids': filteredIds, 'keyboard_submit': true, 'constraint_error': constraintError, 'recovered_output': recovered, 'backend_disconnect_error': disconnectError}));
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpAndSettle();
    expect(tester.takeException(), isNull);
  }, timeout: const Timeout(Duration(minutes: 3)));
}
