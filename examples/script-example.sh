#! /usr/bin/bash
# This example processes summarizes numbers given in a here-document:
cat  | tsvstat <<EOF
A	B	C
12	45	-2
2	4	8
1	7	19
9	88	-3
4	101	7
20	99	5.5
102	36	2
EOF
