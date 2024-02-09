#!/bin/bash

set -e

ls -lrt /

litestream replicate -exec ./rocket -config /etc/litestream.yml
