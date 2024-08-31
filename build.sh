#!/bin/bash

# author Leviathenn

gcc -c lib/caccess.c -o lib/caccess.o
ar rcs lib/libcaccess.a lib/caccess.o
rm lib/caccess.o

if [ $? -ne 0 ]; then 
    echo "Failed to build c library"
    exit 1
fi
echo "Successfully built c library"
