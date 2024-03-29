install_tools:
	# sudo apt install -y clang
	cargo install --locked cargo-spellcheck
	cargo install cargo-fuzz
	cargo install cargo-audit
	# https://github.com/EmbarkStudios/cargo-deny
	cargo install --locked cargo-deny
	# https://github.com/rust-fuzz/honggfuzz-rs/blob/master/README.md
	cargo install honggfuzz
	# https://github.com/nabijaczleweli/cargo-update
	cargo install --locked cargo-outdated
	# for linux sudo apt-get install libssl-dev linux-headers-$(uname -r)
	PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo install cargo-tarpaulin
	cargo install cargo-tree
	# sudo apt install graphviz
	cargo install cargo-deps
	# https://github.com/est31/cargo-udeps
	cargo install cargo-udeps --locked
	# https://crates.io/crates/git-cliff
	cargo install git-cliff
	# https://crates.io/crates/cargo-clean-recursive
	cargo install cargo-clean-recursive
	cargo install cargo-cache
	# https://crates.io/crates/cargo-outdated
	cargo install --locked cargo-outdated

update:
	rustup update
	rustup update nightly

clean:
	cargo cache -a
	cargo clean-recursive
	cargo clean
	# git clean -xdf

format:
	cargo fmt

format_check:
	cargo fmt --check

dep:
	cargo tree
	cargo outdated

udep:
	rustup override set nightly
	cargo udeps
	rustup override set stable

changelog:
	git cliff --output CHANGELOG.md

spellcheck:
	cargo spellcheck check
# cargo spellcheck fix

clippy_fail_on_warnings:
	cargo clippy -- -D warnings -A clippy::large_enum_variant -A clippy::too_many_arguments

clippy:
	cargo clippy -- -A clippy::large_enum_variant -A clippy::too_many_arguments

coverage:
	cargo outdated
	cargo tarpaulin --verbose --all-features --workspace --timeout 120

fuzz_test:
	cargo fuzz

test:
	RUST_BACKTRACE=1 cargo test -- --test-threads=10
docs:
# --exclude "cosmwasm*"
	rm -rf docs && cargo doc --target-dir docs --color never --no-deps --open --workspace --release

audit:
	cargo install cargo-audit --features=fix --force
# cargo deny check

schema:
	find ./packages -mindepth 1 -maxdepth 1 -type d -exec bash -c "(cd {} && cargo schema --target-dir .)" \;

# test -> code-analizer -> security audit -> api docs -> changelog -> schema -> format
all: test clippy audit coverage docs changelog schema