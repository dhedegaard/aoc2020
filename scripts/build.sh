#!/bin/sh
set -e -x

docker build -t aoc2020 .

docker run -it --rm -v `pwd`/out:/out aoc2020 sh -c "cp -rv ./out/* /out/."
