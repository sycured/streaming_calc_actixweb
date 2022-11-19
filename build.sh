#!/bin/bash
set -xe

mkimg=$(buildah from scratch)
buildah config --arch amd64 "$mkimg"
buildah config --author='sycured' "$mkimg"
buildah config --label Name='streaming_calc_actixweb' --label org.opencontainers.image.source="https://github.com/sycured/streaming_calc_actixweb" "$mkimg"
buildah config --env APP_IP=0.0.0.0 "$mkimg"
buildah config --env APP_PORT=8080 "$mkimg"
buildah config --port 8080 "$mkimg"
buildah config --cmd '/streaming_calc_actixweb' "$mkimg"
buildah copy "$mkimg" 'target/x86_64-unknown-linux-gnu/release/streaming_calc_actixweb' '/streaming_calc_actixweb'
buildah commit --squash "$mkimg" "scaw"
buildah rm "$mkimg"