CARGO = cargo
JQ = jq

all: Cargo.toml
	$(CARGO) build --release

check: Cargo.toml tests/fixtures/google_search.jsonld tests/fixtures/x_follower.jsonld
	$(CARGO) test

clean: Cargo.toml
	@rm -rf *~ target
	$(CARGO) clean

tests/fixtures/google_search.jsonld: tests/fixtures/google_search.json src/jq/google_search.jq
	$(JQ) -f src/jq/google_search.jq < $< > $@

tests/fixtures/x_follower.jsonld: tests/fixtures/x_follower.json src/jq/x_follower.jq
	$(JQ) -f src/jq/x_follower.jq < $< > $@

.PHONY: all check clean
.SECONDARY:
.SUFFIXES:
