.PHONY: gen-swagger fmt

gen-swagger:
	@cargo run gen-swagger api/swagger.json

fmt:
	@cargo +nightly fmt && cargo clippy --fix --allow-dirty
