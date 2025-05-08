#!/bin/bash

# takes ca. 7 minutes
N=50

for i in $(seq 1 $N); do echo -n "$i," >> data.txt; /usr/bin/time -f "%e" ./Dokumente/programming/rust/learning_rust/04_fibonacci/main $i 2>> data.txt; done



# run this in the terminal to visualize data
# cat data.txt | uplot line -d,

