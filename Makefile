.PHONY: help install uninstall i u

PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
APP_NAME = git-purge

help:
	@echo "Available commands:"
	@echo ""
	@echo "  make build - Builds git-purge app"
	@echo "  make check - performs test, code and format checks"
	@echo "  make install - Install git-purge to $(BINDIR)"
	@echo "  make uninstall - Remove git-purge from $(BINDIR)"
	@echo "  make c - alias for check command"
	@echo "  make i - alias for install command"
	@echo "  make u - alias for uninstall command"
	@echo ""

build:
	@cargo build --release

check:
	cargo test
	cargo check
	cargo clippy -- -D warnings
	cargo fmt -- --check

c: check

install:
	@$(MAKE) build
	@mkdir -p $(BINDIR)
	@cp target/release/$(APP_NAME) $(BINDIR)
	@echo "✅ Done!"

i: install

uninstall:
	@echo "🗑️  Removing $(APP_NAME) from $(BINDIR)..."
	@rm -f $(BINDIR)/$(APP_NAME)
	@echo "✅ Uninstalled successfully!"

u: uninstall

