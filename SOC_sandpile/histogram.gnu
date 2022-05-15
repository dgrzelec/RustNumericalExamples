N=20

# set xrange [0:1]
# set xrange [0.4:0.8]
# set yrange [-0.1: 1.1]
# set yrange [0:N]
 
set xlabel "cascade size"
set ylabel "#"

fname = "cascade_size.txt"


Min = 0.0 # where binning starts
Max = 200.0 # where binning ends
n = 20 # the number of bins
width = (Max-Min)/n # binwidth; evaluates to 1.0
bin(x) = width*(floor((x-Min)/width)+0.5) + Min

plot fname using (bin($2)):(1.0) smooth freq with boxes notitle

    