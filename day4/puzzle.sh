#!/bin/bash

s=197487
e=673251

count=0
for try in $(seq $s $e); do
    # double digit?
    echo $try | grep -E '(00|11|22|33|44|55|66|77|88|99)' | \
        while read -r x; do
            sorted=$(echo $x | grep -o . | sort | tr -d "\n")
            if [ "$x" == "$sorted" ]; then
                echo $x
                ((count++))
            fi
        done
done

echo $count
