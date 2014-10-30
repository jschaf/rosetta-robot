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


.PHONY: generate-rust-files
generate-rust-files: copy-rust-sources
	python -c \
	  'import glob, json; \
	   print(json.dumps(glob.glob("rosettarobot/test/github-rust-rosetta/src/*.rs"), indent=2))' \
	  > _data/rust_files.json

.PHONY: copy-rust-sources
copy-rust-sources:
	cp rosettarobot/test/github-rust-rosetta/src/*.rs _includes
