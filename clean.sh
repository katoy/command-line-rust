#!/usr/bin/env bash

for DIR in [01]*; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo clean -q > /dev/null )
done

echo "Done."
