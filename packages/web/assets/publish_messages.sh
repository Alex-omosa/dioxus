#!/bin/bash

# Script to read JSON messages from all .json files in the current directory
# and publish each message in the top-level array to NATS subject a2ui.ui

for json_file in *.json; do
  if [ -f "$json_file" ]; then
    echo "Processing $json_file..."
    # Extract each element from the array and publish
    jq -c '.[]' "$json_file" | while read -r msg; do
      nats pub a2ui.ui "$msg"
    done
  fi
done

echo "Done."
