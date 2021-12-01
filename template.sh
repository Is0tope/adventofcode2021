#!/bin/bash

NUM=$1
if [ -z "$NUM" ]
then
    echo "Must provide a day number (e.g. 01)"
    exit
fi
FOLDER=day$NUM
cargo new $FOLDER
touch $FOLDER/test.txt
touch $FOLDER/input.txt
cp source.txt $FOLDER/src/main.rs