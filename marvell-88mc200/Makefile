all:
	svd2rust -i 88mc200.svd
	form -i lib.rs -o src/
	rm lib.rs
	cargo fmt

clean:
	rm -rf build.rs lib.rs device.x src/

.PHONY: all
