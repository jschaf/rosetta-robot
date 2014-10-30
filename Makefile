.PHONY test
test:
	nosetests

.PHONY doc
doc:
	rustdoc  -o output.js --output-format json --no-defaults doctest.rs
