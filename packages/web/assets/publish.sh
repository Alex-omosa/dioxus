#!/usr/bin/env bash
# publish_a2ui.sh
#
# Publishes each A2UI v0.9 message line from a2ui_messages.json
# to the NATS subject "a2ui.ui" in order, with a short delay between them.
#
# Requirements: nats CLI  (https://github.com/nats-io/natscli)
# Usage:
#   chmod +x publish_a2ui.sh
#   ./publish_a2ui.sh
#   ./publish_a2ui.sh --server nats://localhost:4222 --delay 0.5

set -euo pipefail

# ── defaults ──────────────────────────────────────────────────────────────────
SERVER="${NATS_URL:-nats://localhost:4222}"
DELAY=0.3        # seconds between messages
SUBJECT="a2ui.ui"
MESSAGES_FILE="$(dirname "$0")/a2ui_messages.json"

# ── argument parsing ───────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    --server)  SERVER="$2";  shift 2 ;;
    --delay)   DELAY="$2";   shift 2 ;;
    --subject) SUBJECT="$2"; shift 2 ;;
    --file)    MESSAGES_FILE="$2"; shift 2 ;;
    *) echo "Unknown argument: $1"; exit 1 ;;
  esac
done

# ── sanity checks ──────────────────────────────────────────────────────────────
if ! command -v nats &>/dev/null; then
  echo "❌  'nats' CLI not found. Install from https://github.com/nats-io/natscli"
  exit 1
fi

if [[ ! -f "$MESSAGES_FILE" ]]; then
  echo "❌  Messages file not found: $MESSAGES_FILE"
  exit 1
fi

echo "🚀  Publishing A2UI messages to subject '${SUBJECT}'"
echo "    Server  : ${SERVER}"
echo "    File    : ${MESSAGES_FILE}"
echo "    Delay   : ${DELAY}s between messages"
echo "────────────────────────────────────────────────"

seq=0
while IFS= read -r line; do
  # Skip blank lines and comment lines
  [[ -z "$line" || "$line" == \#* ]] && continue

  seq=$((seq + 1))

  # || true prevents grep's non-zero exit (no match) from killing the script
  msg_type=$(echo "$line" | grep -oP '"(createSurface|updateComponents|updateDataModel|deleteSurface)"' | head -1 | tr -d '"' || true)
  [[ -z "$msg_type" ]] && msg_type="unknown"

  echo "📨  [${seq}] ${msg_type}"

  nats pub "$SUBJECT" --server "$SERVER" "$line"

  [[ "${DELAY}" != "0" ]] && sleep "$DELAY"

done < "$MESSAGES_FILE"

echo "────────────────────────────────────────────────"
echo "✅  Done — published ${seq} message(s) to '${SUBJECT}'"