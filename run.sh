# Copyright (c) 2020 PowerSnail
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

name="day$1-part$2"
answer_file="answer/day$1-part$2.txt"
input_file="input/day$1.txt"

cargo build --release
target/release/aoc_2020 $name < $input_file | tee $answer_file | xsel -ib
cat $answer_file