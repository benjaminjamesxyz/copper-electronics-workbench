import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'backend_client.dart';
import 'models.dart';

void main() => runApp(const CopperApp());
const ink = Color(0xff172d36);
const copper = Color(0xff94613b);
const paper = Color(0xfff5f4ef);

class CopperApp extends StatelessWidget {
  const CopperApp({super.key});
  @override
  Widget build(BuildContext context) => MaterialApp(
    title: 'Copper · Electronics workbench', debugShowCheckedModeBanner: false,
    theme: ThemeData(
      useMaterial3: true, scaffoldBackgroundColor: paper,
      colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xff216957), surface: paper),
      textTheme: ThemeData.light().textTheme.apply(bodyColor: ink, displayColor: ink),
      inputDecorationTheme: const InputDecorationTheme(border: OutlineInputBorder(), filled: true, fillColor: Colors.white),
    ),
    home: const Workbench(),
  );
}
class Workbench extends StatefulWidget {
  const Workbench({super.key});
  @override
  State<Workbench> createState() => _WorkbenchState();
}
class _WorkbenchState extends State<Workbench> {
  BackendClient? _backend;
  List<Calculator> _catalog = [];
  Calculator? _selected;
  final Map<String, TextEditingController> _inputs = {};
  final _form = GlobalKey<FormState>();
  String _query = '', _category = 'All domains', _level = 'All levels';
  String? _startupError, _calculationError;
  Map<String, double>? _results;
  bool _loading = true, _calculating = false;
  int _revision = 0;
  Object? _activeCalculation;

  @override
  void initState() { super.initState(); unawaited(_connect()); }
  Future<void> _connect() async {
    BackendClient? backend;
    try {
      backend = await BackendClient.start();
      final response = await backend.request({'command': 'catalog'});
      final catalog = (response['calculations'] as List).map((c) => Calculator(c as Map<String, dynamic>)).toList();
      if (!mounted) { await backend.close(); return; }
      if (catalog.isEmpty) throw StateError('The Rust catalog is empty.');
      setState(() { _backend = backend; _catalog = catalog; _loading = false; });
      _select(catalog.first);
    } catch (error) {
      await backend?.close();
      if (mounted) setState(() { _startupError = error.toString(); _loading = false; });
    }
  }
  void _select(Calculator calculator) {
    for (final controller in _inputs.values) { controller.dispose(); }
    _inputs.clear();
    for (final input in calculator.inputs) { _inputs[input.key] = TextEditingController(text: input.defaultValue.toString()); }
    setState(() {
      _selected = calculator; _results = null; _calculationError = null;
      _calculating = false; _activeCalculation = null; _revision++;
    });
  }
  void _invalidate() => setState(() { _revision++; _results = null; _calculationError = null; });
  void _fillExample() {
    for (final entry in _selected!.exampleInputs.entries) { _inputs[entry.key]!.text = entry.value.toString(); }
    _invalidate();
  }
  Future<void> _calculate() async {
    if (!_form.currentState!.validate()) { _invalidate(); return; }
    final calculator = _selected!;
    final revision = _revision;
    final requestToken = Object();
    final values = _inputs.map((key, controller) => MapEntry(key, double.parse(controller.text.trim())));
    setState(() { _activeCalculation = requestToken; _calculating = true; _calculationError = null; _results = null; });
    try {
      final response = await _backend!.request({'command': 'calculate', 'id': calculator.id, 'inputs': values});
      if (mounted && revision == _revision) {
        setState(() { _results = (response['outputs'] as Map<String, dynamic>).map((key, value) => MapEntry(key, (value as num).toDouble())); });
      }
    } catch (error) {
      if (mounted && revision == _revision) setState(() { _calculationError = error.toString(); });
    } finally {
      if (mounted && identical(_activeCalculation, requestToken)) {
        setState(() { _activeCalculation = null; _calculating = false; });
      }
    }
  }
  @override
  void dispose() {
    for (final controller in _inputs.values) { controller.dispose(); }
    unawaited(_backend?.close());
    super.dispose();
  }
  List<Calculator> get _filtered => _catalog.where((c) =>
    (_category == 'All domains' || c.category == _category) &&
    (_level == 'All levels' || c.level == _level) &&
    '${c.title} ${c.description} ${c.id} ${c.category}'.toLowerCase().contains(_query.toLowerCase())).toList();
  @override
  Widget build(BuildContext context) => Scaffold(
    body: Column(children: [
      Container(color: ink, padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 20), child: Row(children: [
        const Icon(Icons.memory_outlined, color: Color(0xffd7b38b), size: 32), const SizedBox(width: 14),
        const Text('COPPER', style: TextStyle(color: Colors.white, fontSize: 24, letterSpacing: 3, fontWeight: FontWeight.w700)),
        const SizedBox(width: 24), const Expanded(child: Text('ELECTRONICS DESIGN WORKBENCH', style: TextStyle(color: Color(0xffb8c9c8), fontSize: 12, letterSpacing: 1.5))),
        if (!_loading && _startupError == null) Text('${_catalog.length} calculations · Offline', style: const TextStyle(color: Color(0xffb8c9c8))),
      ])),
      Expanded(child: _loading ? const Center(child: CircularProgressIndicator()) : _startupError != null
        ? Center(child: Padding(padding: const EdgeInsets.all(40), child: SelectableText(_startupError!, style: const TextStyle(color: Colors.red))))
        : LayoutBuilder(builder: (context, constraints) {
          if (constraints.maxWidth < 760) {
            return Column(children: [SizedBox(height: 300, child: _catalogPanel()), Expanded(child: _detail())]);
          }
          return Row(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
            SizedBox(width: 310, child: _catalogPanel()), const VerticalDivider(width: 1), Expanded(child: _detail()),
          ]);
        })),
      Container(padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 8), child: const Row(children: [
        Icon(Icons.info_outline, size: 14, color: copper), SizedBox(width: 8),
        Expanded(child: Text('Engineering estimates, not certification. Check model limits and manufacturer specifications.', style: TextStyle(fontSize: 11, color: copper))),
      ])),
    ]),
  );
  Widget _catalogPanel() {
    final categories = ['All domains', ...(_catalog.map((c) => c.category).toSet().toList()..sort())];
    final filtered = _filtered;
    return Material(color: const Color(0xffeeeee7), child: Column(children: [
      Padding(padding: const EdgeInsets.all(20), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        const Text('CALCULATION LIBRARY', style: TextStyle(fontSize: 11, letterSpacing: 1.5, fontWeight: FontWeight.bold, color: copper)), const SizedBox(height: 14),
        TextField(key: const ValueKey('catalog-search'), decoration: const InputDecoration(labelText: 'Search calculations', prefixIcon: Icon(Icons.search), isDense: true), onChanged: (value) => setState(() { _query = value; })),
        const SizedBox(height: 12),
        DropdownButtonFormField<String>(initialValue: _category, isExpanded: true, decoration: const InputDecoration(labelText: 'Domain', isDense: true),
          items: categories.map((c) => DropdownMenuItem(value: c, child: Text(c, overflow: TextOverflow.ellipsis))).toList(), onChanged: (c) => setState(() { _category = c!; })),
        const SizedBox(height: 12),
        DropdownButtonFormField<String>(initialValue: _level, decoration: const InputDecoration(labelText: 'Experience', isDense: true),
          items: ['All levels', 'basic', 'intermediate', 'expert'].map((l) => DropdownMenuItem(value: l, child: Text(l))).toList(), onChanged: (l) => setState(() { _level = l!; })),
        const SizedBox(height: 16), Text('${filtered.length} matching calculations', style: const TextStyle(fontSize: 12)),
      ])),
      Expanded(child: filtered.isEmpty ? const Center(child: Text('No matching calculations.')) : ListView.builder(
        itemCount: filtered.length, itemBuilder: (context, index) {
          final c = filtered[index];
          return ListTile(key: ValueKey('select-${c.id}'), selected: _selected?.id == c.id,
            selectedTileColor: const Color(0xffd8e6dd), contentPadding: const EdgeInsets.symmetric(horizontal: 20, vertical: 4),
            title: Text(c.title, style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600)),
            subtitle: Text('${c.category} · ${c.level}', style: const TextStyle(fontSize: 11)),
            onTap: () => _select(c));
        })),
    ]));
  }
  Widget _detail() {
    final c = _selected;
    if (c == null) return const SizedBox.shrink();
    return SingleChildScrollView(key: const ValueKey('detail-scroll'), padding: const EdgeInsets.all(28), child: Align(alignment: Alignment.topLeft, child: ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 1050), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Text('${c.category.toUpperCase()} / ${c.level.toUpperCase()}', style: const TextStyle(color: copper, letterSpacing: 1.4, fontSize: 11, fontWeight: FontWeight.w600)),
        const SizedBox(height: 8), Text(c.title, style: const TextStyle(fontSize: 30, fontWeight: FontWeight.w600)),
        const SizedBox(height: 10), Text(c.description, style: const TextStyle(fontSize: 15, height: 1.5)),
        const SizedBox(height: 28),
        Container(padding: const EdgeInsets.all(22), decoration: BoxDecoration(color: Colors.white, border: Border.all(color: const Color(0xffd8ddd6)), borderRadius: BorderRadius.circular(8)), child:
          Form(key: _form, child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Row(children: [const Expanded(child: Text('Design inputs', style: TextStyle(fontSize: 18, fontWeight: FontWeight.w600))), TextButton.icon(key: const ValueKey('load-example'), onPressed: _fillExample, icon: const Icon(Icons.science_outlined, size: 18), label: const Text('Load worked example'))]),
            const SizedBox(height: 18),
            LayoutBuilder(builder: (context, constraints) => Wrap(spacing: 18, runSpacing: 18, children: c.inputs.map((input) => SizedBox(
              width: constraints.maxWidth > 560 ? (constraints.maxWidth - 18) / 2 : constraints.maxWidth,
              child: TextFormField(key: ValueKey('input-${input.key}'), controller: _inputs[input.key],
                decoration: InputDecoration(labelText: input.label, suffixText: input.unit, helperText: input.range),
                keyboardType: const TextInputType.numberWithOptions(decimal: true, signed: true),
                onChanged: (_) => _invalidate(), onFieldSubmitted: (_) { if (!_calculating) unawaited(_calculate()); },
                validator: (text) {
                  final value = double.tryParse(text?.trim() ?? '');
                  if (value == null || !value.isFinite) return 'Enter a finite number (e.g. 1e-6).';
                  if (input.min != null && (input.exclusiveMin ? value <= input.min! : value < input.min!)) return 'Required: ${input.range}';
                  if (input.max != null && (input.exclusiveMax ? value >= input.max! : value > input.max!)) return 'Required: ${input.range}';
                  return null;
                },
              ),
            )).toList())),
            const SizedBox(height: 24), FilledButton.icon(key: const ValueKey('calculate'), onPressed: _calculating ? null : _calculate,
              icon: const Icon(Icons.calculate_outlined), label: Text(_calculating ? 'Calculating…' : 'Calculate')),
          ]))),
        const SizedBox(height: 24),
        if (_calculationError != null) Container(key: const ValueKey('calculation-error'), padding: const EdgeInsets.all(16), color: const Color(0xffffe8df), child: SelectableText(_calculationError!, style: const TextStyle(color: Color(0xff8b2818)))),
        if (_results != null) ...[
          const Text('Calculated result', style: TextStyle(fontSize: 18, fontWeight: FontWeight.w600)), const SizedBox(height: 12),
          Wrap(spacing: 16, runSpacing: 12, children: c.outputs.map((o) => Container(
            constraints: const BoxConstraints(minWidth: 210), padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(color: const Color(0xffe0ece2), border: Border.all(color: const Color(0xffb3ccb8)), borderRadius: BorderRadius.circular(6)),
            child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [Text(o.label, style: const TextStyle(fontSize: 12)), const SizedBox(height: 8),
              SelectableText('${_format(_results![o.key]!)} ${o.unit}', key: ValueKey('result-${o.key}'), style: const TextStyle(fontSize: 24, fontWeight: FontWeight.w600))]),
          )).toList()), const SizedBox(height: 28),
        ],
        const Divider(), const SizedBox(height: 14),
        const Text('Model & formula', style: TextStyle(fontSize: 18, fontWeight: FontWeight.w600)), const SizedBox(height: 12),
        SelectableText(c.formula, style: const TextStyle(fontFamily: 'monospace', fontSize: 14, height: 1.6)), const SizedBox(height: 14),
        ...c.assumptions.map((a) => Padding(padding: const EdgeInsets.only(bottom: 8), child: Text('• $a', style: const TextStyle(fontSize: 13, height: 1.5)))),
        const SizedBox(height: 12), ExpansionTile(tilePadding: EdgeInsets.zero, title: const Text('Worked example & references', style: TextStyle(fontSize: 15)), children: [
          Align(alignment: Alignment.centerLeft, child: SelectableText('Expected example outputs: ${c.outputs.map((o) => '${o.label}: ${_format(c.exampleExpected[o.key]!)} ${o.unit}').join('; ')}', style: const TextStyle(fontSize: 13))),
          const SizedBox(height: 12),
          ...c.sources.map((s) => ListTile(contentPadding: EdgeInsets.zero, title: Text(s.title, style: const TextStyle(fontSize: 13)), subtitle: SelectableText(s.url, style: const TextStyle(fontSize: 12)), trailing: IconButton(tooltip: 'Copy reference URL', icon: const Icon(Icons.copy_outlined, size: 18), onPressed: () => Clipboard.setData(ClipboardData(text: s.url))))),
        ]),
      ]),
    )));
  }
  String _format(double value) {
    if (value == 0) return '0';
    if (value.abs() < 0.001 || value.abs() >= 1000000) return value.toStringAsExponential(6);
    return value.toStringAsPrecision(7);
  }
}
