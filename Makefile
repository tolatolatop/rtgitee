# Build target
build:
	@cargo build

# Run target
run:
	@cargo run

# Clean target
clean:
	@cargo clean

# Help target
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build   - Build the project"
	@echo "  run     - Run the project"
	@echo "  clean   - Clean the build artifacts"
	@echo "  help    - Show this help message"