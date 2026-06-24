single HTML page that can help generate the adaptative pressure advance measurements table

only use HTML, CSS and jascript in a single web page


# Example genereated table

the table is similar to this one

Speed	Flow	Acceleration	PA	Model values
50	3.84	1000	0.036	0.036 , 3.84 , 1000
100	7.68	1000	0.036	0.036 , 7.68 , 1000
150	11.51	1000	0.036	0.036 , 11.51 , 1000
200	15.35	1000	0.036	0.036 , 15.35 , 1000
50	3.84	2000	0.036	0.036 , 3.84 , 2000
100	7.68	2000	0.03	0.03 , 7.68 , 2000
150	11.51	2000	0.029	0.029 , 11.51 , 2000
200	15.35	2000	0.028	0.028 , 15.35 , 2000
50	3.84	4000	0.032	0.032 , 3.84 , 4000
100	7.68	4000	0.028	0.028 , 7.68 , 4000
150	11.51	4000	0.026	0.026 , 11.51 , 4000
200	15.35	4000	0.024	0.024 , 15.35 , 4000

# Example generated config

0.036 , 3.84 , 1000
0.036 , 7.68 , 1000
0.036 , 11.51 , 1000
0.036 , 15.35 , 1000
0.036 , 3.84 , 2000
0.03 , 7.68 , 2000
0.029 , 11.51 , 2000
0.028 , 15.35 , 2000
0.032 , 3.84 , 4000
0.028 , 7.68 , 4000
0.026 , 11.51 , 4000
0.024 , 15.35 , 4000

The config is just a text file
There must be a copy button next to the config so it can be copied to the clipboard


# Inputs

Speeds
- a list of the speeds
- a list of accelerations

The generated table should contain all combinations of speeds and accelerations
FLow and PA are generated as blank, and must be set manually
