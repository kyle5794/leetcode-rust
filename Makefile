N=
.phony: p
p:
	@if [ -d "src/${N}" ]; then \
		echo "Problem files already exist"; \
		exit 0; \
	fi
	@mkdir "src/${N}"
	@echo "pub mod solution;" > "src/${N}/mod.rs"
	@echo "## Problem\n\r## Solution" > "src/${N}/readme.md"
	@echo "pub struct Solution{}" > "src/${N}/solution.rs"
	@echo "pub mod ${N};" >> src/lib.rs

.phony: run
run:
	cargo build && cargo run

