#!/bin/bash

arr_size=0

while read line; do
	array=("${array[@]}" $line)
	((arr_size+=1))
done < input

x=1
y=2
done=0

while [[ $done -eq 0 ]]; do
	xval=${array[x]}
	yval=${array[y]}
	t=$((xval+yval))
	if [[ $t -eq 2020 ]]; then
		echo $xval
		echo $yval
		echo $((xval*yval))
		((done+=1))
	else
		if [[ $y -lt arr_size-1 ]]; then
			((y+=1))
		else
			((x+=1))
			y=$x+1
		fi
	fi
done

