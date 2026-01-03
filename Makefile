.PHONY: help
help:
	@echo "Usage:"
	@echo "  make new abc 341                : Create environment for ABC341"
	@echo "  make run abc 341 a              : Run solution for ABC341 problem A"
	@echo "  make run abc 341 a p            : Paste from clipboard and run"
	@echo "  make run abc 341 a release      : Run solution in release mode"
	@echo "  make data                       : Generate test data"
	@echo "  make use src/tmpl.rs abc 341 a  : Inject template"

INPUT_FILE = input.txt

# Keywords to be excluded from positional arguments
KEYWORDS = new run use data release p

# Extract arguments that are not make targets or functional keywords
ARGS = $(filter-out $(KEYWORDS) $@,$(MAKECMDGOALS))

# Mode detection
CARGO_FLAGS =
MODE = debug
ifneq (,$(filter release,$(MAKECMDGOALS)))
	CARGO_FLAGS := --release
	MODE := release
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
	$(eval PKG_NAME := $(T)$(C))
	@# Paste if "p" keyword exists
	@if [ -n "$(DO_PASTE)" ]; then \
		powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE); \
		echo "Copied clipboard content to $(INPUT_FILE)"; \
	fi
	@# Compile using workspace package and bin name
	@cargo build $(CARGO_FLAGS) --quiet -p $(PKG_NAME) --bin $(P)
	@# Execute
	@BIN_PATH="./target/$(MODE)/$(P)"; \
	if [ -f "$${BIN_PATH}.exe" ]; then BIN_PATH="$${BIN_PATH}.exe"; fi; \
	if [ "$(MODE)" = "release" ]; then \
		echo "Start: $$(date +'%H:%M:%S.%3N')"; \
		START_TIME=$$(python -c 'import time; print(time.time())'); \
		if [ -f $(INPUT_FILE) ]; then cat $(INPUT_FILE) | "$$BIN_PATH"; else "$$BIN_PATH"; fi; \
		RET=$$?; \
		END_TIME=$$(python -c 'import time; print(time.time())'); \
		echo "End:   $$(date +'%H:%M:%S.%3N')"; \
		python -c "print(f'Execution time: {$$END_TIME - $$START_TIME:.4f}s')"; \
	else \
		if [ -f $(INPUT_FILE) ]; then cat $(INPUT_FILE) | "$$BIN_PATH"; else "$$BIN_PATH"; fi; \
		RET=$$?; \
	fi; \
	exit $$RET

.PHONY: data
data:
	@python make_data.py > $(INPUT_FILE)
	@echo "Generated test data to $(INPUT_FILE)"

.PHONY: use
use:
	$(eval TMPL := $(word 1, $(ARGS)))
	$(eval T := $(word 2, $(ARGS)))
	$(eval C := $(word 3, $(ARGS)))
	$(eval P := $(word 4, $(ARGS)))
	@if [ -z "$(TMPL)" ] || [ -z "$(T)" ] || [ -z "$(C)" ] || [ -z "$(P)" ]; then \
		echo "Error: Usage: make use <tmpl> <type> <id> <prob>"; exit 1; \
	fi
	$(eval TARGET_FILE := src/$(T)/$(T)$(C)/$(P).rs)
	@python scripts/inject.py "$(TMPL)" "$(TARGET_FILE)"

.PHONY: paste
paste:
	@powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE)
	@echo "Copied clipboard content to $(INPUT_FILE)"

%:
	@: