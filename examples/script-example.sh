#! /usr/bin/bash
# This example processes summarizes numbers given in a here-document:
cat  | tsvstat <<EOF
A	B	C	D	E	F	G	H	I
12	45	-2	1	-1	1.7	-1.12	Y	0
2	4	8	2	-2	2.4	-2	N	1
1	7	19	3	-3	3	-3	Y	1
9	88	-3	4	-4	4	-4	N	0
4	101	7	5	-5	5	-5	Y	1
20	99	5.5	6	-6	6	-6	N	1
102	36	2	7	-7	7	-7	Y	0
EOF
