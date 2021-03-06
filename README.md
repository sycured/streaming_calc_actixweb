# streaming_calc_actixweb

Bandwidth calculation for streaming server | Rewrite from my original in Python

[![CI](https://github.com/sycured/streaming_calc_actixweb/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sycured/streaming_calc_actixweb/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sycured/streaming_calc_actixweb/branch/main/graph/badge.svg)](https://codecov.io/gh/sycured/streaming_calc_actixweb)

## Compilation

    cargo build --release

## Usage

### Run the server

	./target/release/streaming_calc_actixweb

By defaut, it's listening on 127.0.0.1:8080 but it can be tuned via env var:

- APP_IP: define ip address
- APP_PORT: define port

Arguments available: nothing

### Information about endpoints

	curl http://localhost:8080
	curl http://localhost:8080/bwserver
	curl http://localhost:8080/serverusagebw

### Determine necessary server bandwidth

    curl -XPOST -H "Content-Type: application/json" --data '{"nblisteners":250,"bitrate":64}' http://localhost:8080/bwserver

**Output**

    {"result":15625.0}

### Determine the amount of data used for the streaming

    curl -XPOST -H "Content-Type: application/json" --data '{"nblisteners":250,"bitrate":64,"nbdays":1,"nbhours":24}' http://localhost:8080/serverusagebw

**Output**

    {"result":164794.92}