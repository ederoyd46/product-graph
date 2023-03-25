# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)

CROSS_TARGET=x86_64-unknown-linux-musl
CROSS_COMPILE=x86_64-linux-musl-

CURRENT_TAG_VERSION=$(shell git log --format="%h" -n 1)

# Tasks
.PHONY: deploy

# Build Locally
build: 
	@cargo build 

run: 
	@cargo run

test:
	@cargo test

release:
ifeq ("$(UNAME_S)","Linux")
	@cargo build --target=$(CROSS_TARGET) --release
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release
endif

create.fly.app:
	@fly apps create --name ederoyd-product-graph

build.fly.image: release
	@docker build -f ./infrastructure/fly/Dockerfile.GraphQL --tag registry.fly.io/ederoyd-product-graph:$(CURRENT_TAG_VERSION) .

deploy.fly.image:
	@docker push registry.fly.io/ederoyd-product-graph:$(CURRENT_TAG_VERSION)
	@flyctl deploy -c ./infrastructure/fly/fly.toml -i registry.fly.io/ederoyd-product-graph:$(CURRENT_TAG_VERSION) -a ederoyd-product-graph

start.db:
	surreal start -u root -p root -- file://./data

start.mem.db:
	surreal start -u root -p root

connect.db:
	surreal sql -c http://localhost:8000 -u root -p root --ns test --db test --pretty