# Default target
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make new 341 abc               : Create environment for ABC341"
	@echo "  make new 108 arc               : Create environment for ARC108"
	@echo "  make run 341 a abc             : Run solution for ABC341 problem A"
	@echo "  make prun 341 a abc            : Paste input from clipboard and run solution"
	@echo "  make clear 341 abc             : Remove environment for ABC341"
	@echo "  make use src/tmpl.rs 341 a abc : Inject template"
	@echo "  make open 341 a abc            : Open solution for ABC341 problem A in VSCode"

INPUT_FILE = input.txt

# Get list of arguments excluding the target itself ($@)
ARGS = $(filter-out $@,$(MAKECMDGOALS))

# Variables to capture arguments
ARG1 = $(word 1, $(ARGS))
ARG2 = $(word 2, $(ARGS))
ARG3 = $(word 3, $(ARGS))
ARG4 = $(word 4, $(ARGS))

# Create contest environment
# Usage: make new <contest_id> <contest_type>
.PHONY: new
new:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then \
		echo "Error: Contest ID and Type required. (e.g., make new 341 abc)"; \
		exit 1; \
	fi
	@./mkrs.sh $(ARG1) $(ARG2)

# Run solution
# Usage: make run <contest_id> <problem_id> <contest_type>
.PHONY: run
run:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ]; then \
		echo "Error: Contest ID, Problem ID, and Type required. (e.g., make run 341 a abc)"; \
		exit 1; \
	fi
	$(eval C := $(ARG1))
	$(eval P := $(ARG2))
	$(eval T := $(ARG3))
	$(eval PREFIX := $(T)$(C))
	@if [ -f $(INPUT_FILE) ]; then \
		cat $(INPUT_FILE) | cargo run --quiet --bin $(PREFIX)_$(P); \
	else \
		echo "[ Manual Input Mode ]"; \
		cargo run --quiet --bin $(PREFIX)_$(P); \
	fi

# Paste from clipboard and Run solution
# Usage: make prun <contest_id> <problem_id> <contest_type>
.PHONY: prun
prun:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ]; then \
		echo "Error: Contest ID, Problem ID, and Type required. (e.g., make prun 341 a abc)"; \
		exit 1; \
	fi
	@# Paste logic
	@powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE)
	@echo "Copied clipboard content to $(INPUT_FILE)"
	@# Run logic
	$(eval C := $(ARG1))
	$(eval P := $(ARG2))
	$(eval T := $(ARG3))
	$(eval PREFIX := $(T)$(C))
	@cat $(INPUT_FILE) | cargo run --quiet --bin $(PREFIX)_$(P)

# Remove environment
# Usage: make clear <contest_id> <contest_type>
.PHONY: clear
clear:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then \
		echo "Error: Contest ID and Type required. (e.g., make clear 341 abc)"; \
		exit 1; \
	fi
	@./rmrs.sh $(ARG1) $(ARG2)

# Inject template
# Usage: make use <template_path> <contest_id> <problem_id> <contest_type>
.PHONY: use
use:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ] || [ -z "$(ARG4)" ]; then \
		echo "Error: Usage: make use <template_path> <contest_id> <problem_id> <contest_type>"; \
		exit 1; \
	fi
	$(eval TEMPLATE_PATH := $(ARG1))
	$(eval C := $(ARG2))
	$(eval P := $(ARG3))
	$(eval T := $(ARG4))
	$(eval TARGET_FILE := src/$(T)/$(C)/$(P).rs)
	@python scripts/inject.py "$(TEMPLATE_PATH)" "$(TARGET_FILE)"

# Open in VSCode
# Usage: make open <contest_id> <problem_id> <contest_type>
.PHONY: open
open:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ]; then \
		echo "Error: Contest ID, Problem ID, and Type required. (e.g., make open 341 a abc)"; \
		exit 1; \
	fi
	$(eval C := $(ARG1))
	$(eval P := $(ARG2))
	$(eval T := $(ARG3))
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

# Dummy rule to prevent arguments from being interpreted as targets
%:
	@: