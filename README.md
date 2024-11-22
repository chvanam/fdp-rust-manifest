## Benchmarking

```bash
# Install asyncapi-cli
npm install -g @asyncapi/cli

# Install hyperfine
apt install hyperfine
brew install hyperfine

# Install datamodel-code-generator
pip install datamodel-code-generator

# Install graphviz (dot)
apt install graphviz
brew install graphviz

# Build using cargo
cd fdp-core
cargo build --release

# Run benchmarks
cd ../benchmarks
./bench.sh
```

### Number of messages

In `benchmarks/bench.sh`, update the scripts to point to the `/examples/asyncapi/{1|5|30}_message.yaml` files.
In `fdp-core/fdp-definition/src/apps/mod.rs`, update the `app_1` and `app_2` modules to match the new number of messages.
