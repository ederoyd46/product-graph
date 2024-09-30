# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)

CROSS_TARGET=x86_64-unknown-linux-musl

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
	cargo zigbuild --release --target $(CROSS_TARGET)

create.fly.app:
	@fly apps create --name ederoyd-product-graph
	@fly auth docker
	@fly ips allocate-v4 --shared
	@fly ips allocate-v6 --private #for fly cast only way to get 2 process groups to talk to each other at the moment
	@fly volume create ederoyd_product_graph_db --region lhr --size 3  -y
	@fly secrets set DATABASE_URL="ederoyd-product-graph.flycast:8000" #This will be available as an environment variable

build.fly.image: release
	@docker buildx build --platform linux/amd64 -f ./infrastructure/fly/Dockerfile.GraphQL --tag registry.fly.io/ederoyd-product-graph:$(CURRENT_TAG_VERSION) .

deploy.fly.image: build.fly.image
	@docker push registry.fly.io/ederoyd-product-graph:$(CURRENT_TAG_VERSION)
	@flyctl deploy -c ./infrastructure/fly/fly.toml -i registry.fly.io/ederoyd-product-graph:$(CURRENT_TAG_VERSION) -a ederoyd-product-graph

start.db:
	surreal start -u root -p root -- surrealkv://./data

start.mem.db:
	surreal start -u root -p root

connect.db:
	surreal sql -e http://localhost:8000 --ns test --db test -u root -p root --pretty

connect.db.remote:
	surreal sql -e http://ederoyd-product-graph.flycast:8000 --ns test --db test -u root -p root --pretty
