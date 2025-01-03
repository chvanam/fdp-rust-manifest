#!/bin/bash

# Temporary benchmark directory, make it writable
OUTPUT_DIR="temp"
mkdir -p $OUTPUT_DIR
chmod -R u+w $OUTPUT_DIR

# Output file for benchmark results
OUTPUT_FILE="benchmark_results.json"

# Define functions for benchmarking
asyncapi_pydantic() {
    asyncapi generate models python ../examples/asyncapi/1_message.yaml --pyDantic -o temp/pydantic
}

asyncapi_doc() {
    asyncapi generate fromTemplate ../examples/asyncapi/1_message.yaml @asyncapi/html-template -o temp/docs --force-write --use-new-generator
}

asyncapi_python() {
    asyncapi generate fromTemplate ../examples/asyncapi/1_message.yaml @asyncapi/python-paho-template -o temp/python --force-write
}

rust_doc(){
    cargo run --manifest-path ../fdp-core/Cargo.toml --package fdp-definition --release --bin graph
}

rust_python(){
    cargo run --manifest-path ../fdp-core/Cargo.toml --package fdp-definition --release --bin python -- --output temp/gen
}

# Export functions
# export -f asyncapi_pydantic
export -f asyncapi_doc
export -f asyncapi_python
export -f rust_doc
export -f rust_python

# Run benchmarks
echo "Running benchmarks"
hyperfine --shell=bash --warmup 2 --runs 8 \
    'rust_python' \
    'asyncapi_python' \
    'rust_doc' \
    'asyncapi_doc' \
    --export-json $OUTPUT_FILE \

echo "Benchmarks completed"
echo "Results saved in $OUTPUT_FILE"

# Generate whisker plot
# python ../benchmarks/scripts/plot_whisker.py ../benchmarks/$OUTPUT_FILE
