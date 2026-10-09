#!/usr/bin/env python3
"""Exercise the actual Rust JSON-lines process against independent research examples."""
import json
import math
import subprocess
import sys
from pathlib import Path

root = Path(__file__).resolve().parents[1]
binary = Path(sys.argv[1]) if len(sys.argv) > 1 else root / 'backend/target/debug/pcb-calculator'
process = subprocess.Popen([str(binary.resolve())], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)

def request(payload):
    process.stdin.write(json.dumps(payload) + '\n')
    process.stdin.flush()
    response = process.stdout.readline()
    if not response:
        raise RuntimeError('Rust engine closed without a response')
    return json.loads(response)

failures = []
try:
    catalog = request({'command': 'catalog'})
    if not catalog['ok']:
        raise RuntimeError(catalog['error'])
    calculations = catalog['calculations']
    for calculation in calculations:
        example = calculation['example']
        response = request({'command': 'calculate', 'id': calculation['id'], 'inputs': example['inputs']})
        if not response['ok']:
            failures.append(f"{calculation['id']}: {response['error']}")
            continue
        outputs = response['outputs']
        if set(outputs) != set(example['expected']):
            failures.append(f"{calculation['id']}: example output keys differ")
            continue
        tolerance = example['relative_tolerance']
        for key, expected in example['expected'].items():
            actual = outputs[key]
            limit = tolerance * max(abs(expected), 1e-300)
            if not math.isfinite(actual) or abs(actual - expected) > limit:
                failures.append(f"{calculation['id']}.{key}: expected {expected}, observed {actual}, tolerance {limit}")
    # A bad request must not poison the following request or stop the process.
    invalid = request({'command': 'calculate', 'id': 'no.such.calculator', 'inputs': {}})
    assert invalid['ok'] is False and invalid['error']
    process.stdin.write('{malformed\n')
    process.stdin.flush()
    assert json.loads(process.stdout.readline())['ok'] is False
    assert request({'command': 'catalog'})['ok'] is True
finally:
    process.stdin.close()
    process.wait(timeout=10)
if failures:
    print('\n'.join(failures))
    raise SystemExit(1)
levels = {level: sum(c['level'] == level for c in calculations) for level in ('basic', 'intermediate', 'expert')}
print(f"PASS: {len(calculations)} calculators exercised through the Rust process; levels={levels}; error recovery passed")
