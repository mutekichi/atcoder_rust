# デフォルトのターゲット
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make new c=341       : Create environment for ABC341"
	@echo "  make new c=108 t=arc : Create environment for ARC108"
	@echo "  make run c=341 p=a   : Run solution for ABC341 problem A (uses input.txt if exists)"
	@echo "  make clear c=341     : Remove environment for ABC341"
	@echo "  make use name=union_find c=341 p=a : Inject union_find template into ABC341 problem A"

# 変数設定
t ?= abc
prefix = $(t)$(c)
INPUT_FILE = input.txt

# コンテスト環境作成
.PHONY: new
new:
	@./mkrs.sh $(c) $(t)

# 実行 (改良版)
# input.txt が存在すればそれを入力として与え、なければ手動入力を待つ
.PHONY: run
run:
	@if [ -f $(INPUT_FILE) ]; then \
		cat $(INPUT_FILE) | cargo run --quiet --bin $(prefix)_$(p); \
	else \
		echo "[ Manual Input Mode ]"; \
		cargo run --quiet --bin $(prefix)_$(p); \
	fi

# 削除
.PHONY: clear
clear:
	@./rmrs.sh $(c) $(t)

# Usage: make use name=union_find c=341 p=a (t=abc)
.PHONY: use
use:
	@if [ -z "$(name)" ]; then \
		echo "Error: Template name is required. (e.g., make use name=union_find ...)"; \
		exit 1; \
	fi
	@if [ -z "$(p)" ]; then \
		echo "Error: Problem ID is required. (e.g., p=a)"; \
		exit 1; \
	fi
	@python scripts/injection.py $(name) src/$(t)/$(c)/$(p).rs