N=16

# set xrange [0:1]
set xrange [0.4:0.8]
set yrange [-0.1: 1.1]
# set yrange [0:N]
set grid

set key left top 
set xlabel "p"
set ylabel "W(p;L)"

fname_1 = "p_plot_L17.txt"
# fname_2 = "p_plot_L33.txt"
# fname_3 = "p_plot_L65.txt"
# fname_4 = "p_plot_L129.txt"

plot fname_1 with lines lw 2 title "L=16",\
    # fname_2 with lines  lw 2 title "L=32", \
    # fname_3 with lines  lw 2 title "L=64", \
    # fname_4 with lines  lw 2 title "L=128"
    