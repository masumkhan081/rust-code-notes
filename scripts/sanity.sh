#!/usr/bin/env bash
# scripts/sanity.sh — compile-gate for rust-code-notes
# Run this before committing to ensure the repo is in a clean state.
# Exit code: 0 = all gates passed, non-zero = at least one gate failed.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PASS=0
FAIL=0

run_gate() {
    local label="$1"
    shift
    printf "  %-40s" "$label"
    if "$@" > /tmp/sanity_out.txt 2>&1; then
        echo "PASS"
        PASS=$((PASS + 1))
    else
        echo "FAIL"
        cat /tmp/sanity_out.txt
        FAIL=$((FAIL + 1))
    fi
}

echo "======================================================"
echo " rust-code-notes sanity check"
echo "======================================================"

# ── 1. format check ───────────────────────────────────────────────────────
run_gate "cargo fmt --check (notes)"          cargo fmt -p notes    -- --check
run_gate "cargo fmt --check (axum_minimal)"   cargo fmt -p axum_minimal -- --check

# ── 2. build ──────────────────────────────────────────────────────────────
run_gate "cargo build -p notes"               cargo build -p notes
run_gate "cargo build -p axum_minimal"        cargo build -p axum_minimal

# ── 3. clippy ─────────────────────────────────────────────────────────────
run_gate "cargo clippy -p notes"              cargo clippy -p notes          -- -D warnings
run_gate "cargo clippy -p axum_minimal"       cargo clippy -p axum_minimal   -- -D warnings

# ── 4. tests ──────────────────────────────────────────────────────────────
run_gate "cargo test -p notes"                cargo test -p notes
run_gate "cargo test -p axum_minimal"         cargo test -p axum_minimal

# ── 5. no committed binaries ──────────────────────────────────────────────
printf "  %-40s" "no binary artifacts in repo"
BINS=$(find . -not -path './.git/*' -not -path './target/*' \
       \( -name '*.exe' -o -name '*.out' -o -name '*.bin' \) 2>/dev/null)
if [ -z "$BINS" ]; then
    echo "PASS"
    PASS=$((PASS + 1))
else
    echo "FAIL"
    echo "$BINS"
    FAIL=$((FAIL + 1))
fi

echo "======================================================"
echo " PASSED: $PASS   FAILED: $FAIL"
echo "======================================================"

[ "$FAIL" -eq 0 ]
