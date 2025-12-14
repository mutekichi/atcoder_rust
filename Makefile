# デフォルトのターゲット
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make new 341       : Create environment for ABC341"
	@echo "  make new 108 arc   : Create environment for ARC108"
	@echo "  make run 341 a     : Run solution for ABC341 problem A"
	@echo "  make clear 341     : Remove environment for ABC341"
	@echo "  make use src/templates/union_find.rs 341 a : Inject template"
	@echo "  make open 341 a    : Open solution for ABC341 problem A in VSCode"

# デフォルトのコンテスト種類
DEFAULT_TYPE = abc
INPUT_FILE = input.txt

# コマンドライン引数からターゲット自体($@)を除いたリストを取得
ARGS = $(filter-out $@,$(MAKECMDGOALS))

# 引数の取得用変数 (1番目, 2番目, 3番目, 4番目)
ARG1 = $(word 1, $(ARGS))
ARG2 = $(word 2, $(ARGS))
ARG3 = $(word 3, $(ARGS))
ARG4 = $(word 4, $(ARGS))

# コンテスト環境作成
# Usage: make new <contest_id> [contest_type]
.PHONY: new
new:
	@if [ -z "$(ARG1)" ]; then echo "Error: Contest ID required."; exit 1; fi
	@./mkrs.sh $(ARG1) $(if $(ARG2),$(ARG2),$(DEFAULT_TYPE))

# 実行
# Usage: make run <contest_id> <problem_id> [contest_type]
.PHONY: run
run:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then echo "Error: Contest ID and Problem ID required."; exit 1; fi
	$(eval C := $(ARG1))
	$(eval P := $(ARG2))
	$(eval T := $(if $(ARG3),$(ARG3),$(DEFAULT_TYPE)))
	$(eval PREFIX := $(T)$(C))
	@if [ -f $(INPUT_FILE) ]; then \
		cat $(INPUT_FILE) | cargo run --quiet --bin $(PREFIX)_$(P); \
	else \
		echo "[ Manual Input Mode ]"; \
		cargo run --quiet --bin $(PREFIX)_$(P); \
	fi

# 削除
# Usage: make clear <contest_id> [contest_type]
.PHONY: clear
clear:
	@if [ -z "$(ARG1)" ]; then echo "Error: Contest ID required."; exit 1; fi
	@./rmrs.sh $(ARG1) $(if $(ARG2),$(ARG2),$(DEFAULT_TYPE))

# テンプレート挿入
# Usage: make use <template_path> <contest_id> <problem_id> [contest_type]
.PHONY: use
use:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ] || [ -z "$(ARG3)" ]; then \
		echo "Error: Usage: make use <template_path> <contest_id> <problem_id> [contest_type]"; \
		exit 1; \
	fi
	$(eval TEMPLATE_PATH := $(ARG1))
	$(eval C := $(ARG2))
	$(eval P := $(ARG3))
	$(eval T := $(if $(ARG4),$(ARG4),$(DEFAULT_TYPE)))
	$(eval TARGET_FILE := src/$(T)/$(C)/$(P).rs)
	@python scripts/inject.py "$(TEMPLATE_PATH)" "$(TARGET_FILE)"

# VSCode で開く
# Usage: make open <contest_id> <problem_id> [contest_type]
.PHONY: open
open:
	@if [ -z "$(ARG1)" ] || [ -z "$(ARG2)" ]; then echo "Error: Contest ID and Problem ID required."; exit 1; fi
	$(eval C := $(ARG1))
	$(eval P := $(ARG2))
	$(eval T := $(if $(ARG3),$(ARG3),$(DEFAULT_TYPE)))
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

# 引数として渡された文字列（数値やパスなど）がターゲットとして解釈されエラーになるのを防ぐダミールール
%:
	@: