.PHONY: all

all: 
	DEFMT_LOG=info cargo build 
clean: 
	DEFMT_LOG=info cargo clean
run: 
	DEFMT_LOG=info cargo run
led: 
	cd examples/ && DEFMT_LOG=info cargo run --bin led
i2c: 
	DEFMT_LOG=info cargo run --bin i2c
cam: 
	cd examples/
	DEFMT_LOG=info cargo run --bin cam
