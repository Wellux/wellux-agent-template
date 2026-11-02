.PHONY: check test fmt personal business desktop skill doctor shell project

project:
	@if [ -z "$(NAME)" ] && [ -z "$(REPO)" ]; then echo "Usage: make project NAME=my-agent [REPO=https://github.com/owner/my-agent]"; exit 1; fi
	@./scripts/new-project.sh "$(NAME)" "$(REPO)"

shell:
	cargo run -p wellux-shell

check:
	cargo check --workspace --all-targets

test:
	cargo test --workspace --all-targets

fmt:
	cargo fmt --all -- --check

personal:
	cargo build -p agent-core --features memory-engram

business:
	cargo build -p agent-core --features otel,mcp,memory-engram,sandbox-microvm

desktop:
	cargo build -p agent-core --features computer-use,sandbox-microvm,sandbox-quicksand,memory-engram

skill:
	@test -n "$(NAME)" || (echo "Usage: make skill NAME=my-skill" && exit 1)
	@./scripts/new-skill.sh "$(NAME)"

doctor:
	@./scripts/doctor.sh
