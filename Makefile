all: build check test docs

fmt:
	cargo fmt


build: full default add-ordinal-suffix commas digits find-capital-by-province persian-chars national-id remove-ordinal-suffix url-fix verity-card-number time-ago phone-number bill number-to-words get-bank-name-by-card-number extract-card-number get-place-by-iran-national-id half-space legal-id words-to-number

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

persian-chars:
	@ echo ""
	cargo build --no-default-features --features=persian-chars
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

url-fix:
	@ echo ""
	cargo build --no-default-features --features=url-fix
	@ ls -sh target/debug/*.rlib

verity-card-number:
	@ echo ""
	cargo build --no-default-features --features=verity-card-number
	@ ls -sh target/debug/*.rlib
	
time-ago:
	@ echo ""
	cargo build --no-default-features --features=time-ago
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

number-to-words:
	@ echo ""
	cargo build --no-default-features --features=number-to-words

	@ ls -sh target/debug/*.rlib

get-bank-name-by-card-number:
	@ echo ""
	cargo build --no-default-features --features=get-bank-name-by-card-number
	@ ls -sh target/debug/*.rlib

extract-card-number:
	@ echo ""
	cargo build --no-default-features --features=extract-card-number
	@ ls -sh target/debug/*.rlib

get-place-by-iran-national-id:
	@ echo ""
	cargo build --no-default-features --features=get-place-by-iran-national-id
	@ ls -sh target/debug/*.rlib

half-space:
	@ echo ""
	cargo build --no-default-features --features=half-space
	@ ls -sh target/debug/*.rlib

legal-id:
	@ echo ""
	cargo build --no-default-features --features=legal-id
words-to-number:
	@ echo ""
	cargo build --no-default-features --features=words-to-number
	@ ls -sh target/debug/*.rlib