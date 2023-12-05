.PHONY: all

all: 
	DEFMT_LOG=info cargo build 
clean: 
	cd examples/ && DEFMT_LOG=info cargo clean
run: 
	DEFMT_LOG=info cargo run
led: 
	cd examples/ && DEFMT_LOG=info cargo run --bin led
i2c: 
	cd examples/ && DEFMT_LOG=info cargo run --bin i2c
cam: 
	cd examples/ && DEFMT_LOG=info cargo run --bin cam
sd: 
	cd examples/ && DEFMT_LOG=info cargo run --bin sd
