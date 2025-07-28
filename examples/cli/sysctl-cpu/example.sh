#!/bin/sh

./sysctl-cpu |
	dasel --read=json --write=toml --colour
