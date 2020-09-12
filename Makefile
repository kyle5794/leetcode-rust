N=
.phony: p
p:
	@if [ -d "src/prob_${N}" ]; then \
		echo "Problem files already exist"; \
		exit 0; \
	fi
	@mkdir "src/prob_${N}"
	@echo "pub mod solution;" > "src/prob_${N}/mod.rs"
	@touch "src/prob_${N}/readme.md"
	@echo "pub struct Solution{};" >  "src/prob_${N}/solution.rs"
	@echo "\npub mod prob_${N};" >> src/lib.rs
	

