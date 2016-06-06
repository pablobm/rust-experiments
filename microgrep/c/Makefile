microgrep: target/microgrep

target/microgrep: src/microgrep.c target
	cc -o target/microgrep src/microgrep.c

run: microgrep
	./target/microgrep ${INPUT_PATH}

target:
	mkdir target
