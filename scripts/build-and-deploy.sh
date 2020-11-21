#!/bin/sh
set -e -x

# Remove any existing builds.
rm -rf ./web/out

# Build the image.
docker build -t aoc2020 .

# Copy the build from the image to a linked volume.
docker run \
  -it \
  --rm \
  -v `pwd`/web/out:/out \
  aoc2020 \
  sh -c "cp -r ./out/* /out/. && chown -R $(id -u ${USER}):$(id -g ${USER}) /out"

# Deploy the output.
netlify deploy --dir=./web/out --prod
