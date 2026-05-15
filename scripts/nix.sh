#!/usr/bin/env bash

nix --extra-experimental-features 'nix-command flakes' build $link_flag --out-link .nix --allow-dirty "$@"
