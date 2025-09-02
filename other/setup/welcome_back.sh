#!/bin/bash
if [ -e "/sys/module/rosetta" ] && ! mountpoint /root; then mount --bind .root/ /root; fi
git pull
rq -m -g all
