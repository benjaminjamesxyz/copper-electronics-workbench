import 'dart:io';

Future<void> main() async {
  final platform = Platform.isLinux ? 'linux' : Platform.isMacOS ? 'macos' : Platform.isWindows ? 'windows' : null;
  if (platform == null) throw UnsupportedError('Desktop only: Linux, macOS or Windows required.');
  final frontend = File.fromUri(Platform.script).parent.parent;
  final root = frontend.parent;
  Future<void> run(String executable, List<String> arguments, String directory) async {
    final process = await Process.start(executable, arguments, workingDirectory: directory, runInShell: Platform.isWindows);
    await Future.wait([stdout.addStream(process.stdout), stderr.addStream(process.stderr)]);
    final code = await process.exitCode;
    if (code != 0) exit(code);
  }
  await run('cargo', ['build', '--release', '--locked'], '${root.path}/backend');
  await run('flutter', ['build', platform, '--release'], frontend.path);
  final name = Platform.isWindows ? 'pcb-calculator.exe' : 'pcb-calculator';
  final String destination;
  if (platform == 'linux') {
    final architecture = Platform.version.contains('arm64') ? 'arm64' : 'x64';
    destination = '${frontend.path}/build/linux/$architecture/release/bundle/$name';
  } else if (platform == 'windows') {
    destination = '${frontend.path}/build/windows/x64/runner/Release/$name';
  } else {
    destination = '${frontend.path}/build/macos/Build/Products/Release/pcb_design_calculator.app/Contents/MacOS/$name';
  }
  await File('${root.path}/backend/target/release/$name').copy(destination);
  stdout.writeln('Desktop bundle includes Rust engine: $destination');
}
