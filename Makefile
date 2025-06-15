########################################
## Name - Ritankar Saha
## Email - ritankar.saha786@gmail.com
########################################

.PHONY: help build test clean run deploy fmt clippy doc bench

# Default target
help:
	@echo "MediToken Rust Implementation"
	@echo "============================="
	@echo ""
	@echo "Available commands:"
	@echo "  build        Build the project"
	@echo "  test         Run all tests"
	@echo "  clean        Clean build artifacts"
	@echo "  run          Run the test suite"
	@echo "  deploy       Deploy to specified network"
	@echo "  fmt          Format code"
	@echo "  clippy       Run clippy linter"
	@echo "  doc          Generate documentation"
	@echo "  bench        Run benchmarks"
	@echo ""
	@echo "Deployment examples:"
	@echo "  make deploy NETWORK=sepolia"
	@echo "  make deploy NETWORK=arbitrum"
	@echo "  make deploy NETWORK=optimism"

# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test --verbose

# Clean build artifacts
clean:
	cargo clean

# Run the test runner binary
run:
	cargo run --bin test_runner

# Deploy to network (requires NETWORK parameter)
deploy:
ifndef NETWORK
	@echo "Error: NETWORK parameter required"
	@echo "Usage: make deploy NETWORK=<network>"
	@echo "Available networks: sepolia, amoy, arbitrum, optimism, cardona, scroll, local"
	@exit 1
endif
	@echo "Deploying to $(NETWORK)..."
	cargo run --bin deploy -- --network $(NETWORK) --verbose

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Generate documentation
doc:
	cargo doc --open

# Run benchmarks (if any)
bench:
	cargo bench

# Install dependencies
install:
	@echo "Installing Rust dependencies..."
	cargo check

# Update dependencies
update:
	cargo update

# Development setup
setup: install
	@echo "Setting up development environment..."
	@cp .env.example .env
	@echo "Please configure your .env file with appropriate values"

# All-in-one development command
dev: fmt clippy test