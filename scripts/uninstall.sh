#!/bin/sh
set -e

NAME="terb"

echo "Removing $NAME..."

sudo rm -f "/usr/local/bin/$NAME"

echo "Done!"
