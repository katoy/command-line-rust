#!/usr/bin/env bash

for DIR in [01]*; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo clippy )
done

echo "Done."
