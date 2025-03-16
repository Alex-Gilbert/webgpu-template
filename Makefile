# Makefile for RenderDoc debugging on Arch Linux
# Project name (defaults to directory name if not specified)
PROJECT_NAME := demo 

# RenderDoc command line utility
RENDERDOC_CMD := renderdoccmd

# Default target
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	cargo build

# Build with RenderDoc integration
.PHONY: build-debug
build-debug:
	cargo build --features debug-renderdoc

# Get the path to the built executable
EXECUTABLE := target/debug/$(PROJECT_NAME)

# Run the application normally
.PHONY: run
run: build
	./$(EXECUTABLE)

# Debug with RenderDoc via remote server
.PHONY: debug-renderdoc-server
debug-renderdoc-server: build-debug
	@echo "Starting RenderDoc remote server..."
	@($(RENDERDOC_CMD) remoteserver & echo $$! > .renderdoc.pid)
	@sleep 1
	@echo "RenderDoc started with PID: $$(cat .renderdoc.pid)"
	@echo "Launching application with RenderDoc integration..."
	./$(EXECUTABLE)
	@echo "Application closed, shutting down RenderDoc..."
	@kill $$(cat .renderdoc.pid) && rm .renderdoc.pid || true

# Debug with RenderDoc direct capture (simpler approach)
.PHONY: debug-renderdoc
debug-renderdoc: build-debug
	@echo "Launching application with RenderDoc capture..."
	RUST_BACKTRACE=1 \
	RENDERDOC_CAPTUREALLOPENGL=1 \
	RENDERDOC_CAPTUREALLVULKAN=1 \
	WGPU_BACKEND=vulkan \
	$(RENDERDOC_CMD) capture -w ./$(EXECUTABLE)

# Clean the project
.PHONY: clean
clean:
	cargo clean
	rm -f .renderdoc.pid

# Start the application first, then attach RenderDoc
.PHONY: debug-renderdoc-attach
debug-renderdoc-attach: build-debug
	@echo "Starting application first..."
	@./$(EXECUTABLE) & echo $$! > .app.pid
	@sleep 2
	@echo "Now attach RenderDoc manually to PID $$(cat .app.pid)"
	@$(RENDERDOC_CMD) inject -PID $$(cat .app.pid)

# Try a different approach for maximum compatibility
.PHONY: debug-renderdoc-direct
debug-renderdoc-direct: build-debug
	@echo "Launching application directly with RenderDoc..."
	@echo "Note: You'll need to manually capture frames using F12 or your configured hotkey"
	RUST_BACKTRACE=1 \
	RENDERDOC_CAPTUREALLOPENGL=1 \
	RENDERDOC_CAPTUREALLVULKAN=1 \
	WGPU_BACKEND=vulkan \
	$(RENDERDOC_CMD) capture \
		--working-dir "$(CURDIR)" \
		--capture-all-cmd ./$(EXECUTABLE)@rm .app.pid
