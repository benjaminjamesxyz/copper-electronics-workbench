import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:pcb_design_calculator/backend_client.dart';
import 'package:pcb_design_calculator/main.dart';
import 'package:pcb_design_calculator/models.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets('real Rust examples render at all experience levels; edits clear stale results', (tester) async {
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
    await waitFor(find.byKey(const ValueKey('catalog-search')));
    for (final level in ['basic', 'intermediate', 'expert']) {
      final calculator = catalog.firstWhere((c) => c.level == level);
      await tester.enterText(find.byKey(const ValueKey('catalog-search')), calculator.id);
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(ValueKey('select-${calculator.id}')));
      await tester.pumpAndSettle();
      final example = find.byKey(const ValueKey('load-example'));
      await tester.ensureVisible(example);
      await tester.pumpAndSettle();
      await tester.tap(example);
      await tester.pumpAndSettle();
      final calculate = find.byKey(const ValueKey('calculate'));
      await tester.ensureVisible(calculate);
      await tester.pumpAndSettle();
      await tester.tap(calculate);
      final firstResult = find.byKey(ValueKey('result-${calculator.outputs.first.key}'));
      await waitFor(firstResult);
      for (final output in calculator.outputs) {
        final rendered = tester.widget<SelectableText>(find.byKey(ValueKey('result-${output.key}'))).data!;
        final value = double.parse(rendered.split(' ').first);
        final expected = calculator.exampleExpected[output.key]!;
        expect(value, closeTo(expected, expected.abs() * 1e-5 + 1e-300));
      }
      final firstInput = find.byKey(ValueKey('input-${calculator.inputs.first.key}'));
      await tester.ensureVisible(firstInput);
      await tester.enterText(firstInput, 'NaN');
      await tester.pumpAndSettle();
      expect(firstResult, findsNothing, reason: 'Changed inputs must invalidate the previous result');
      await tester.ensureVisible(calculate);
      await tester.pumpAndSettle();
      await tester.tap(calculate);
      await tester.pumpAndSettle();
      expect(find.text('Enter a finite number (e.g. 1e-6).'), findsWidgets);
      expect(firstResult, findsNothing);
    }
    await tester.enterText(find.byKey(const ValueKey('catalog-search')), 'data.adc_lsb');
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('select-data.adc_lsb')));
    await tester.pumpAndSettle();
    final bits = find.byKey(const ValueKey('input-bits'));
    await tester.ensureVisible(bits);
    await tester.enterText(bits, '32');
    final calculate = find.byKey(const ValueKey('calculate'));
    await tester.ensureVisible(calculate);
    await tester.pumpAndSettle();
    await tester.tap(calculate);
    final count = find.byKey(const ValueKey('result-code_count'));
    await waitFor(count);
    final rendered = tester.widget<SelectableText>(count).data!;
    expect(double.parse(rendered.split(' ').first), 4294967296.0,
      reason: 'A discrete 32-bit code count must not lose hundreds of codes to display rounding');
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpAndSettle();
  });
}
