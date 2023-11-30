.PHONY: all

all: 
	DEFMT_LOG=info cargo build 
clean: 
	DEFMT_LOG=info cargo clean
run: 
	DEFMT_LOG=info cargo run
led: 
	DEFMT_LOG=info cargo run --bin led
