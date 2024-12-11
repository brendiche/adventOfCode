#!/bin/bash

echo "Enter the day number:"
read day
cargo new day$day
mkdir day$day/assets
touch day$day/assets/input
touch day$day/assets/input_test
cd day$day
codium .