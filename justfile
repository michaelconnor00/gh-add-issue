# gh-add-issue justfile

binary := "gh-add-issue"
debug_binary := "target/debug/" + binary

# Build the debug binary and refresh the root symlink
build:
    cargo build
    ln -sf {{debug_binary}} {{binary}}

# Build a release binary and refresh the root symlink
build-release:
    cargo build --release
    ln -sf target/release/{{binary}} {{binary}}

# Install (or reinstall) the extension from the local directory
install: build
    gh extension install . 2>/dev/null || gh extension install --force . 

# Run the extension (passes any extra args through)
run *ARGS: build
    gh add-issue {{ARGS}}

# Run all tests
test:
    cargo test

# Lint with clippy
lint:
    cargo clippy -- -D warnings

# Format source code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt -- --check

# Remove build artefacts and the root symlink
clean:
    cargo clean
    rm -f {{binary}}
