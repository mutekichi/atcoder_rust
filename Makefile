.PHONY: help
help:
	@echo "Usage:"
	@echo "  make new abc 341                : Create environment for ABC341"
	@echo "  make run abc 341 a              : Run solution for ABC341 problem A"
	@echo "  make run abc 341 a p            : Paste from clipboard and run"
	@echo "  make run abc 341 a release      : Run solution in release mode"
	@echo "  make data                       : Generate test data"
	@echo "  make clear abc 341              : Remove environment"
	@echo "  make use src/tmpl.rs abc 341 a  : Inject template"
	@echo "  make open abc 341 a             : Open solution in VSCode"

INPUT_FILE = input.txt

# Keywords to be excluded from positional arguments
KEYWORDS = new run clear use open data release p

# Extract arguments that are not make targets or functional keywords
ARGS = $(filter-out $(KEYWORDS) $@,$(MAKECMDGOALS))

# Mode detection
CARGO_FLAGS =
BIN_DIR = target/debug
ifneq (,$(filter release,$(MAKECMDGOALS)))
	CARGO_FLAGS := --release
	BIN_DIR := target/release
endif

# Check if "p" is provided as an argument
DO_PASTE = $(filter p,$(MAKECMDGOALS))

.PHONY: release p
release p:
	@:

.PHONY: new
new:
	$(eval T := $(word 1, $(ARGS)))
	$(eval C := $(word 2, $(ARGS)))
	@if [ -z "$(T)" ] || [ -z "$(C)" ]; then echo "Error: Type and ID required."; exit 1; fi
	@./mkrs.sh $(C) $(T)

.PHONY: run
run:
	$(eval T := $(word 1, $(ARGS)))
	$(eval C := $(word 2, $(ARGS)))
	$(eval P := $(word 3, $(ARGS)))
	@if [ -z "$(T)" ] || [ -z "$(C)" ] || [ -z "$(P)" ]; then echo "Error: Type, ID, and Prob required."; exit 1; fi
	$(eval PREFIX := $(T)$(C))
	@# Paste if "p" keyword exists
	@if [ -n "$(DO_PASTE)" ]; then \
		powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE); \
		echo "Copied clipboard content to $(INPUT_FILE)"; \
	fi
	@# Compile
	@cargo build $(CARGO_FLAGS) --quiet --bin $(PREFIX)_$(P)
	@# Execute
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

.PHONY: data
data:
	@python make_data.py > $(INPUT_FILE)
	@echo "Generated test data to $(INPUT_FILE)"

.PHONY: clear
clear:
	$(eval T := $(word 1, $(ARGS)))
	$(eval C := $(word 2, $(ARGS)))
	@if [ -z "$(T)" ] || [ -z "$(C)" ]; then echo "Error: Type and ID required."; exit 1; fi
	@./rmrs.sh $(C) $(T)

.PHONY: use
use:
	$(eval TMPL := $(word 1, $(ARGS)))
	$(eval T := $(word 2, $(ARGS)))
	$(eval C := $(word 3, $(ARGS)))
	$(eval P := $(word 4, $(ARGS)))
	@if [ -z "$(TMPL)" ] || [ -z "$(T)" ] || [ -z "$(C)" ] || [ -z "$(P)" ]; then \
		echo "Error: Usage: make use <tmpl> <type> <id> <prob>"; exit 1; \
	fi
	$(eval TARGET_FILE := src/$(T)/$(C)/$(P).rs)
	@python scripts/inject.py "$(TMPL)" "$(TARGET_FILE)"

.PHONY: open
open:
	$(eval T := $(word 1, $(ARGS)))
	$(eval C := $(word 2, $(ARGS)))
	$(eval P := $(word 3, $(ARGS)))
	$(eval TARGET_PATH := src/$(T)/$(C)/$(P).rs)
	@if [ -f "$(TARGET_PATH)" ]; then \
		code "$(TARGET_PATH)"; \
	else \
		echo "Error: File '$(TARGET_PATH)' does not exist."; exit 1; \
	fi

.PHONY: paste
paste:
	@powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE)
	@echo "Copied clipboard content to $(INPUT_FILE)"

%:
	@: