all: build check test docs

fmt:
	cargo fmt


build: full default add-ordinal-suffix commas digits find-capital-by-province is-persian national-id remove-ordinal-suffix to-persian-chars url-fix verity-card-number phone-number

check: clippy lint

test:
	RUST_BACKTRACE=1 cargo test --all-features -- --nocapture

docs:
	cargo doc --all-features


clippy:
	cargo clippy --all-features --no-deps

lint:
	cargo fmt --check --verbose


full:
	@ echo ""
	cargo build --no-default-features --features=full
	@ ls -sh target/debug/*.rlib

default:
	@ echo ""
	cargo build
	@ ls -sh target/debug/*.rlib

add-ordinal-suffix:
	@ echo ""
	cargo build --no-default-features --features=add-ordinal-suffix
	@ ls -sh target/debug/*.rlib

commas:
	@ echo ""
	cargo build --no-default-features --features=commas
	@ ls -sh target/debug/*.rlib

digits:
	@ echo ""
	cargo build --no-default-features --features=digits
	@ ls -sh target/debug/*.rlib

find-capital-by-province:
	@ echo ""
	cargo build --no-default-features --features=find-capital-by-province
	@ ls -sh target/debug/*.rlib

is-persian:
	@ echo ""
	cargo build --no-default-features --features=is-persian
	@ ls -sh target/debug/*.rlib

national-id:
	@ echo ""
	cargo build --no-default-features --features=national-id
	@ ls -sh target/debug/*.rlib
	cargo build --no-default-features --features="national-id serde"
	@ ls -sh target/debug/*.rlib


remove-ordinal-suffix:
	@ echo ""
	cargo build --no-default-features --features=remove-ordinal-suffix
	@ ls -sh target/debug/*.rlib

to-persian-chars:
	@ echo ""
	cargo build --no-default-features --features=to-persian-chars
	@ ls -sh target/debug/*.rlib

url-fix:
	@ echo ""
	cargo build --no-default-features --features=url-fix
	@ ls -sh target/debug/*.rlib

verity-card-number:
	@ echo ""
	cargo build --no-default-features --features=verity-card-number
	@ ls -sh target/debug/*.rlib

phone-number:
	@ echo ""
	cargo build --no-default-features --features=phone-number
	@ ls -sh target/debug/*.rlib
	cargo build --no-default-features --features="phone-number serde"
	@ ls -sh target/debug/*.rlib

bill:
	@ echo ""
	cargo build --no-default-features --features=bill
	@ ls -sh target/debug/*.rlib