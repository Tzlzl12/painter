#!/bin/bash

# Script to build and run all examples in the examples/ directory

EXAMPLES_DIR="examples"
FAILED_EXAMPLES=()

echo "Building and running all examples..."

# Build all examples first
echo "Building all examples..."
if ! cargo build --examples; then
	echo "Failed to build examples"
	exit 1
fi

# Run each example
for example_file in "$EXAMPLES_DIR"/*.rs; do
	# Extract the example name (filename without extension)
	example_name=$(basename "$example_file" .rs)

	echo "==============================="
	echo "Running example: $example_name"
	echo "==============================="

	# Run the example
	if cargo run --example "$example_name"; then
		echo "✅ $example_name completed successfully"
	else
		echo "❌ $example_name failed"
		FAILED_EXAMPLES+=("$example_name")
	fi

	echo ""
done

# Summary
echo "==============================="
echo "SUMMARY"
echo "==============================="

if [ ${#FAILED_EXAMPLES[@]} -eq 0 ]; then
	echo "🎉 All examples ran successfully!"
else
	echo "⚠️  Failed examples:"
	for failed in "${FAILED_EXAMPLES[@]}"; do
		echo "  - $failed"
	done
	exit 1
fi
