.PHONY: all
all:
	make build
	make pack
	make graph
	make info


.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run: build
	substreams run substreams.yaml map_balance_changes -e mainnet.eth.streamingfast.io:443

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_balance_changes -e mainnet.eth.streamingfast.io:443

