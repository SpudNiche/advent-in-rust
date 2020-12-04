#!/bin/bash

arr_size=0

while read line; do
	array=("${array[@]}" $line)
	((arr_size+=1))
done < input

x=1
y=2
z=3
done=0

while [[ $done -eq 0 ]]; do
	lil_done=0
	xval=${array[x]}
	
	while [[ $lil_done -eq 0 ]]; do
		yval=${array[y]}
		zval=${array[z]}
		t=$((xval+yval+zval))
		if [[ $t -eq 2020 ]]; then
			echo $xval
			echo $yval
			echo $zval
			echo $((xval*yval*zval))
			((lil_done+=1))
			((done+=1))
		else
			if [[ $z -lt arr_size-1 ]]; then
				((z+=1))
			else 
				((y+=1))
				z=$y+1
			fi

			if [[ $y -eq arr_size-2 ]]; then
				((lil_done+=1))
			fi
		fi
	done

	((x+=1))
	y=$x+1
	z=$y+1
done

