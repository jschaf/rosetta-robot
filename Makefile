.PHONY: test
test:
	nosetests

.PHONY: doc
doc:
	rustdoc  -o output.js --output-format json --no-defaults doctest.rs

.PHONY: update-rust-rosetta
update-rust-rosetta:
	git subtree pull \
	  --prefix rosettarobot/test/github-rust-rosetta \
	  https://github.com/Hoverbear/rust-rosetta.git \
	  master \
	  --squash
