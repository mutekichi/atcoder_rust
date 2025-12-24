# Default target
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make new 341 abc                : Create environment for ABC341"
	@echo "  make new 108 arc                : Create environment for ARC108"
	@echo "  make run 341 abc a              : Run solution for ABC341 problem A"
	@echo "  make run 341 abc a release      : Run solution in release mode"
	@echo "  make prun 341 abc a             : Paste input from clipboard and run solution"
	@echo "  make data                       : Generate test data using make_data.py"
	@echo "  make clear 341 abc              : Remove environment for ABC341"
	@echo "  make use src/tmpl.rs 341 abc a  : Inject template"
	@echo "  make open 341 abc a             : Open solution for ABC341 problem A in VSCode"

INPUT_FILE = input.txt

# Detect release mode (using "release" as a target/keyword)
CARGO_FLAGS =
# Define executable directory based on mode
BIN_DIR = target/debug
ifneq (,$(filter release,$(MAKECMDGOALS)))
	CARGO_FLAGS := --release
	BIN_DIR := target/release
endif

# Get list of arguments excluding the target itself ($@) and keywords
ARGS = $(filter-out $@ release,$(MAKECMDGOALS))

# Variables to capture arguments
ARG1 = $(word 1, $(ARGS))
ARG2 = $(word 2, $(ARGS))
ARG3 = $(word 3, $(ARGS))
ARG4 = $(word 4, $(ARGS))

# Dummy target to allow "make ... release" without error
.PHONY: release
release:
	@:

# Create contest environment
.PHONY: new
new:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then \
		echo "Error: Contest ID and Type required."; \
		exit 1; \
	fi
	@./mkrs.sh $(ARG1) $(ARG2)

# Run solution
.PHONY: run
run:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ]; then \
		echo "Error: Contest ID, Type, and Problem ID required."; \
		exit 1; \
	fi
	$(eval C := $(ARG1))
	$(eval T := $(ARG2))
	$(eval P := $(ARG3))
	$(eval PREFIX := $(T)$(C))
	@# 1. Compile first (exclude from timing)
	@cargo build $(CARGO_FLAGS) --quiet --bin $(PREFIX)_$(P)
	@# 2. Determine binary path and Run in a SINGLE shell block to preserve variables
	@BIN_PATH="./$(BIN_DIR)/$(PREFIX)_$(P)"; \
	if [ -f "$${BIN_PATH}.exe" ]; then BIN_PATH="$${BIN_PATH}.exe"; fi; \
	echo "Starting execution at $$(date +'%H:%M:%S')"; \
	start_time=$$(python -c 'import time; print(time.time())'); \
	if [ -f $(INPUT_FILE) ]; then \
		cat $(INPUT_FILE) | "$$BIN_PATH"; \
	else \
		echo "[ Manual Input Mode ]"; \
		"$$BIN_PATH"; \
	fi; \
	RET=$$?; \
	end_time=$$(python -c 'import time; print(time.time())'); \
	python -c "print(f'Execution time: {$$end_time - $$start_time:.4f}s')"; \
	exit $$RET

# Paste from clipboard and Run solution
.PHONY: prun
prun:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ]; then \
		echo "Error: Contest ID, Type, and Problem ID required."; \
		exit 1; \
	fi
	@# Paste logic
	@powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE)
	@echo "Copied clipboard content to $(INPUT_FILE)"
	$(eval C := $(ARG1))
	$(eval T := $(ARG2))
	$(eval P := $(ARG3))
	$(eval PREFIX := $(T)$(C))
	@# 1. Compile first
	@cargo build $(CARGO_FLAGS) --quiet --bin $(PREFIX)_$(P)
	@# 2. Determine binary path and Run in a SINGLE shell block
	@BIN_PATH="./$(BIN_DIR)/$(PREFIX)_$(P)"; \
	if [ -f "$${BIN_PATH}.exe" ]; then BIN_PATH="$${BIN_PATH}.exe"; fi; \
	echo "Starting execution at $$(date +'%H:%M:%S')"; \
	start_time=$$(python -c 'import time; print(time.time())'); \
	cat $(INPUT_FILE) | "$$BIN_PATH"; \
	RET=$$?; \
	end_time=$$(python -c 'import time; print(time.time())'); \
	python -c "print(f'Execution time: {$$end_time - $$start_time:.4f}s')"; \
	exit $$RET

# Generate test data
.PHONY: data
data:
	@python make_data.py > $(INPUT_FILE)
	@echo "Generated test data from make_data.py to $(INPUT_FILE)"

# Remove environment
.PHONY: clear
clear:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then \
		echo "Error: Contest ID and Type required."; \
		exit 1; \
	fi
	@./rmrs.sh $(ARG1) $(ARG2)

# Inject template
.PHONY: use
use:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ] || [ -z "$(ARG4)" ]; then \
		echo "Error: Usage: make use <template_path> <contest_id> <contest_type> <problem_id>"; \
		exit 1; \
	fi
	$(eval TEMPLATE_PATH := $(ARG1))
	$(eval C := $(ARG2))
	$(eval T := $(ARG3))
	$(eval P := $(ARG4))
	$(eval TARGET_FILE := src/$(T)/$(C)/$(P).rs)
	@python scripts/inject.py "$(TEMPLATE_PATH)" "$(TARGET_FILE)"

# Open in VSCode
.PHONY: open
open:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ]; then \
		echo "Error: Contest ID, Type, and Problem ID required."; \
		exit 1; \
	fi
	$(eval C := $(ARG1))
	$(eval T := $(ARG2))
	$(eval P := $(ARG3))
	$(eval TARGET_PATH := src/$(T)/$(C)/$(P).rs)
	@if [ -f "$(TARGET_PATH)" ]; then \
		code "$(TARGET_PATH)"; \
	else \
		echo "Error: File '$(TARGET_PATH)' does not exist."; \
		exit 1; \
	fi

# Paste clipboard content to input file only
.PHONY: paste
paste:
	@powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE)
	@echo "Copied clipboard content to $(INPUT_FILE)"

# Dummy rule
%:
	@: