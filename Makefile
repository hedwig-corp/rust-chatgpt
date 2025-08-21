.PHONY: build fmt clippy

# Default target
build: inner-build fmt clippy

help:
	@echo "Available targets:"
	@echo "  make build        - Build the project in debug mode"
	@echo "  make release      - Build the project in release mode"
	@echo "  make test         - Run all tests"
	@echo "  make test-ollama  - Run tests with Ollama (requires Ollama running)"
	@echo "  make check        - Run cargo check"
	@echo "  make fmt          - Format code with rustfmt"
	@echo "  make clippy       - Run clippy linter"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make doc          - Generate documentation"
	@echo "  make ci           - Run CI checks (fmt, clippy, check, test)"
	@echo "  make install-ollama - Install and setup Ollama (macOS/Linux)"

# Build targets
inner-build:
	cargo build --verbose

release:
	cargo build --release --verbose

# Test targets
test:
	cargo test --verbose

test-ollama:
	@echo "Running tests with Ollama..."
	@echo "Make sure Ollama is running with: ollama serve"
	OLLAMA_BASE_URL=http://localhost:11434 OLLAMA_API_KEY=ollama cargo test --verbose

# Code quality targets
check:
	cargo check --all-features

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

clippy-fix:
	cargo clippy --fix --allow-dirty --allow-staged

# Clean target
clean:
	cargo clean

# Documentation
doc:
	cargo doc --no-deps --open

doc-all:
	cargo doc --open

# CI pipeline - runs all checks
ci: fmt-check clippy check test
	@echo "✅ All CI checks passed!"

# Development helpers
watch:
	cargo watch -x check -x test

run-example:
	@echo "Running example with Ollama..."
	OLLAMA_BASE_URL=http://localhost:11434 OLLAMA_API_KEY=ollama cargo run --example basic

# Ollama setup helper
install-ollama:
	@echo "Installing Ollama..."
	@if [ "$$(uname)" = "Darwin" ]; then \
		echo "Installing on macOS..."; \
		brew install ollama || echo "Brew not found, please install from https://ollama.ai"; \
	elif [ "$$(uname)" = "Linux" ]; then \
		echo "Installing on Linux..."; \
		curl -fsSL https://ollama.ai/install.sh | sh; \
	else \
		echo "Please install Ollama manually from https://ollama.ai"; \
	fi
	@echo "Starting Ollama service..."
	ollama serve &
	@sleep 5
	@echo "Pulling llama3.2 model..."
	ollama pull llama3.2:latest
	@echo "✅ Ollama setup complete!"

# Cargo.toml version bump helpers
bump-patch:
	@current=$$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2); \
	new=$$(echo $$current | awk -F. '{print $$1"."$$2"."$$3+1}'); \
	sed -i '' "s/version = \"$$current\"/version = \"$$new\"/" Cargo.toml; \
	echo "Version bumped from $$current to $$new"

bump-minor:
	@current=$$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2); \
	new=$$(echo $$current | awk -F. '{print $$1"."$$2+1".0"}'); \
	sed -i '' "s/version = \"$$current\"/version = \"$$new\"/" Cargo.toml; \
	echo "Version bumped from $$current to $$new"

bump-major:
	@current=$$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2); \
	new=$$(echo $$current | awk -F. '{print $$1+1".0.0"}'); \
	sed -i '' "s/version = \"$$current\"/version = \"$$new\"/" Cargo.toml; \
	echo "Version bumped from $$current to $$new"

# Publishing
publish-dry:
	cargo publish --dry-run

publish:
	cargo publish