test:
	nosetests

doc:
	rustdoc  -o output.js --output-format json --no-defaults doctest.rs
