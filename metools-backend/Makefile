DATABASE_URL ?= postgres://postgres:postgres@localhost:5432/metools
JWT_SECRET ?= 123
JWT_MAXAGE ?= 6000

default: run
.PHONY: gen-db-schema
.PHONY: gen-swagger
gen-swagger:
	@cargo run gen-swagger api/swagger.yaml
.PHONY: fmt
fmt:
	@cargo +nightly fmt && cargo clippy --fix --allow-dirty --allow-staged
.PHONY: run
run:
	@DATABASE_URL=${DATABASE_URL} JWT_SECRET=${JWT_SECRET} JWT_MAXAGE=${JWT_MAXAGE} cargo run
.PHONY: run-release
run-release:
	@DATABASE_URL=${DATABASE_URL} JWT_SECRET=${JWT_SECRET} JWT_MAXAGE=${JWT_MAXAGE} cargo run --release
run-db:
	@docker compose -f docker-compose.yaml up pg
