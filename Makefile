CARGO = cargo
JQ = jq

all: Cargo.toml
	$(CARGO) build --release

check: Cargo.toml tests/fixtures/google_search.jsonld
	$(CARGO) test

clean: Cargo.toml
	@rm -rf *~ target
	$(CARGO) clean

tests/fixtures/google_search.jsonld: tests/fixtures/google_search.json src/jq/google_search.jq
	$(JQ) -f src/jq/google_search.jq < $< > $@

.PHONY: all check clean
.SECONDARY:
.SUFFIXES:
