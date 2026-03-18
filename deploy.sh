#!/bin/bash
set -e
./build.sh
echo "Pushing to local registry..."
podman push --tls-verify=false registry.gt.lo:5000/console:edge
echo "Deployed console:edge"
