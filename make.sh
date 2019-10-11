#!/bin/sh
main=rssplugin

gcc -Wall -shared -o lib${main}.so -fPIC plugin.c\
    -Isrc -L. $PWD/target/release/librs${main}.so\
    `pkg-config --cflags --libs libxfce4panel-2.0`\
