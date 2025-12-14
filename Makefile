# デフォルトのターゲット
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make new 341 abc             : Create environment for ABC341"
	@echo "  make new 108 arc             : Create environment for ARC108"
	@echo "  make run 341 a abc           : Run solution for ABC341 problem A"
	@echo "  make clear 341 abc           : Remove environment for ABC341"
	@echo "  make use src/tmpl.rs 341 a abc : Inject template"
	@echo "  make open 341 a abc          : Open solution for ABC341 problem A in VSCode"

INPUT_FILE = input.txt

# コマンドライン引数からターゲット自体($@)を除いたリストを取得
ARGS = $(filter-out $@,$(MAKECMDGOALS))

# 引数の取得用変数
ARG1 = $(word 1, $(ARGS))
ARG2 = $(word 2, $(ARGS))
ARG3 = $(word 3, $(ARGS))
ARG4 = $(word 4, $(ARGS))

# コンテスト環境作成
# Usage: make new <contest_id> <contest_type>
.PHONY: new
new:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then \
		echo "Error: Contest ID and Type required. (e.g., make new 341 abc)"; \
		exit 1; \
	fi
	@./mkrs.sh $(ARG1) $(ARG2)

# 実行
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

# 削除
# Usage: make clear <contest_id> <contest_type>
.PHONY: clear
clear:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then \
		echo "Error: Contest ID and Type required. (e.g., make clear 341 abc)"; \
		exit 1; \
	fi
	@./rmrs.sh $(ARG1) $(ARG2)

# テンプレート挿入
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

# VSCode で開く
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

.PHONY: paste
paste:
	@powershell.exe -command "Get-Clipboard" | tr -d '\r' > $(INPUT_FILE)
	@echo "Copied clipboard content to $(INPUT_FILE)"

# 引数として渡された文字列がターゲットとして解釈されエラーになるのを防ぐダミールール
%:
	@: