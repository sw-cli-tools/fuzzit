#!/usr/bin/env bash
set -euo pipefail

# fuzzit test target with known bugs
# Usage: echo "input" | ./examples/test_target.sh

input=$(cat)

case "$input" in
    panic*)
        echo "panic: deliberate crash" >&2
        exit 101
        ;;
    loop)
        while true; do sleep 0.1; done
        ;;
    "")
        echo "ok: empty"
        exit 0
        ;;
    *)
        if [ ${#input} -gt 100000 ]; then
            echo "error: input too long" >&2
            exit 1
        fi
        echo "ok: ${#input} bytes"
        exit 0
        ;;
esac
