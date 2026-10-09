import 'dart:async';
import 'dart:convert';
import 'dart:io';

/// One private Rust subprocess; no shell, listener, or duplicate formulas.
class BackendClient {
  BackendClient._(this._process) {
    _lines = StreamIterator(_process.stdout.transform(utf8.decoder).transform(const LineSplitter()));
    _stderr = _process.stderr.transform(utf8.decoder).listen((text) {
      _diagnostics = '$_diagnostics$text';
      if (_diagnostics.length > 4096) _diagnostics = _diagnostics.substring(_diagnostics.length - 4096);
    });
  }
  final Process _process;
  late final StreamIterator<String> _lines;
  late final StreamSubscription<String> _stderr;
  Future<void> _queue = Future.value();
  bool _closed = false;
  bool _disposed = false;
  String _diagnostics = '';

  static Future<BackendClient> start() async {
    final override = Platform.environment['PCB_CALCULATOR_BACKEND'];
    final directory = File(Platform.resolvedExecutable).parent.path;
    final name = Platform.isWindows ? 'pcb-calculator.exe' : 'pcb-calculator';
    final path = override ?? '$directory${Platform.pathSeparator}$name';
    if (!File(path).existsSync()) {
      throw StateError('Rust backend not found at $path. Build with dart run tool/build_desktop.dart, or set PCB_CALCULATOR_BACKEND to the compiled executable.');
    }
    return BackendClient._(await Process.start(File(path).absolute.path, const []));
  }

  Future<Map<String, dynamic>> request(Map<String, dynamic> request) {
    final response = _queue.then((_) async {
      if (_closed) throw StateError('Calculation engine is closed.');
      try {
        _process.stdin.writeln(jsonEncode(request));
        await _process.stdin.flush();
        final available = await _lines.moveNext().timeout(const Duration(seconds: 10));
        if (!available) throw StateError('Calculation engine stopped. $_diagnostics');
        final result = jsonDecode(_lines.current) as Map<String, dynamic>;
        if (result['ok'] != true) throw StateError(result['error'] as String? ?? 'Calculation failed.');
        return result;
      } on TimeoutException {
        _closed = true;
        _process.kill();
        throw StateError('Calculation engine did not respond within 10 seconds.');
      }
    });
    _queue = response.then<void>((_) {}, onError: (Object _, StackTrace _) {});
    return response;
  }

  Future<void> close() async {
    if (_disposed) return;
    _disposed = true;
    _closed = true;
    await _process.stdin.close();
    _process.kill();
    await _lines.cancel();
    await _stderr.cancel();
  }
}
