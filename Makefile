CARGO=cargo

all: build
build:
	$(CARGO) build
run:
	$(CARGO) run
release-build:
	$(CARGO) build --release
publish: release-build
	$(CARGO) publish
