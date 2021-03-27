#!/bin/bash
set -xe

mkimg=$(buildah from ubuntu:20.04)
buildah config --author='sycured' "$mkimg"
buildah config --label Name='streaming_calc_actixweb' "$mkimg"
buildah config --env APP_IP=0.0.0.0 "$mkimg"
buildah config --env APP_PORT=8080 "$mkimg"
buildah config --port 8080 "$mkimg"
buildah config --workingdir='/opt' "$mkimg"
buildah config --cmd '/opt/streaming_calc_actixweb' "$mkimg"
buildah copy "$mkimg" 'target/release/streaming_calc_actixweb' '/opt/streaming_calc_actixweb'
buildah copy "$mkimg" 'static' '/opt/static/'
buildah commit --squash "$mkimg" "scaw"
buildah rm "$mkimg"
