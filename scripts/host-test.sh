#!/usr/bin/env bash
# Unattended host-agent entry point. All subprocess output and exit status are kept.
set -euo pipefail
if [[ "${WAGO_TEST_WINDOW:-}" != CLOSED ]]; then
    echo 'HOLD: host builds and tests are disabled. Wait for explicit WINDOW CLOSED from the measurement coordinator.' >&2
    echo 'After that release only, set WAGO_TEST_WINDOW=CLOSED to enable this script.' >&2
    exit 75
fi
repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo"
mkdir -p out/host-run
evidence="$repo/out/host-run"
# Exclude simultaneous runs without a background process or a sleep loop.
if ! mkdir "$evidence/.running" 2>/dev/null; then
    echo 'ERROR: out/host-run/.running exists; check for a live runner before removing it.' >&2
    exit 2
fi
finish() {
    result=$?
    trap - EXIT
    printf '{"exit_status":%d,"evidence":"out/host-run","frame":"out/host-run/frame.png"}\n' "$result" > "$evidence/status.json"
    cat "$evidence/status.json"
    rmdir "$evidence/.running"
    exit "$result"
}
trap finish EXIT
exec > >(tee "$evidence/run.log") 2>&1
rm -f "$evidence/status.json" "$evidence/comparison.json" "$evidence/frame.png" "$evidence/frame.png.json" "$evidence/run.json"
printf 'source_commit=%s\n' "$(git rev-parse HEAD)"
git status --short
case "${1:-}" in
    '')
        for tool in cargo rustup go make glslangValidator python3; do
            command -v "$tool" >/dev/null || { echo "ERROR: required tool missing: $tool"; exit 127; }
        done
        rustc --version
        go version
        rustup target add wasm32-unknown-unknown
        # Rebuild from source so an old executable cannot masquerade as this commit.
        make -B all native
        ;;
    --built)
        command -v python3 >/dev/null
        ;;
    *) echo 'Usage: bash scripts/host-test.sh [--built]'; exit 2 ;;
esac
python3 - "$evidence" <<'PY'
import hashlib, json, pathlib, platform, subprocess, sys
paths = ['guest/build/tracer.wasm', 'host/build/wago-raytracer',
         'guest/native/target/release/wago-raytracer-native', 'guest/src/tracer.rs']
record = {'source_commit': subprocess.check_output(['git','rev-parse','HEAD'], text=True).strip(),
          'source_status': subprocess.check_output(['git','status','--porcelain'], text=True),
          'platform': platform.platform(), 'mode': 'headless offscreen Vulkan',
          'sha256': {p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest() for p in paths}}
pathlib.Path(sys.argv[1], 'build.json').write_text(json.dumps(record, indent=2)+'\n')
PY
./host/build/wago-raytracer -require-gpu -out "$evidence/frame.png" -evidence "$evidence"
python3 - "$evidence" <<'PY'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
report = json.loads((root / 'frame.png.json').read_text())
print(json.dumps({'device': report['device'], 'device_type': report['device_type'],
                  'frame': str(root / 'frame.png'), 'samples': report['samples']}))
if 'RTX 4060' not in report['device'] or report['device_type'] != 2:
    raise SystemExit('ERROR: this test requires the real NVIDIA RTX 4060')
PY
./guest/native/target/release/wago-raytracer-native "$evidence"
python3 scripts/compare-renders.py "$evidence"
python3 - "$evidence" <<'PY'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
report = json.loads((root / 'frame.png.json').read_text())
report.update({'status': 'pass', 'exit_status': 0,
               'frame': str(root / 'frame.png'), 'comparison': str(root / 'comparison.json'),
               'build': str(root / 'build.json')})
(root / 'run.json').write_text(json.dumps(report, indent=2)+'\n')
print(json.dumps(report))
PY
