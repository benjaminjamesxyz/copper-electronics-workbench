class QuantityInput {
  QuantityInput(Map<String, dynamic> json)
      : key = json['key'] as String, label = json['label'] as String,
        unit = json['unit'] as String, defaultValue = (json['default'] as num).toDouble(),
        min = (json['min'] as num?)?.toDouble(), max = (json['max'] as num?)?.toDouble(),
        exclusiveMin = json['exclusive_min'] == true, exclusiveMax = json['exclusive_max'] == true;
  final String key, label, unit;
  final double defaultValue;
  final double? min, max;
  final bool exclusiveMin, exclusiveMax;
  String get range {
    final bounds = <String>[];
    if (min != null) bounds.add('${exclusiveMin ? '>' : '≥'} $min');
    if (max != null) bounds.add('${exclusiveMax ? '<' : '≤'} $max');
    return bounds.isEmpty ? 'Finite number' : bounds.join(', ');
  }
}
class QuantityOutput {
  QuantityOutput(Map<String, dynamic> json)
      : key = json['key'] as String, label = json['label'] as String, unit = json['unit'] as String;
  final String key, label, unit;
}
class Source {
  Source(Map<String, dynamic> json) : title = json['title'] as String, url = json['url'] as String;
  final String title, url;
}
class Calculator {
  Calculator(Map<String, dynamic> json)
      : id = json['id'] as String, title = json['title'] as String,
        category = json['category'] as String, level = json['level'] as String,
        description = json['description'] as String, formula = json['formula'] as String,
        assumptions = (json['assumptions'] as List).cast<String>(),
        sources = (json['references'] as List).map((s) => Source(s as Map<String, dynamic>)).toList(),
        inputs = (json['inputs'] as List).map((i) => QuantityInput(i as Map<String, dynamic>)).toList(),
        outputs = (json['outputs'] as List).map((o) => QuantityOutput(o as Map<String, dynamic>)).toList(),
        exampleInputs = (json['example']['inputs'] as Map<String, dynamic>).map((k, v) => MapEntry(k, (v as num).toDouble())),
        exampleExpected = (json['example']['expected'] as Map<String, dynamic>).map((k, v) => MapEntry(k, (v as num).toDouble()));
  final String id, title, category, level, description, formula;
  final List<String> assumptions;
  final List<Source> sources;
  final List<QuantityInput> inputs;
  final List<QuantityOutput> outputs;
  final Map<String, double> exampleInputs, exampleExpected;
}
