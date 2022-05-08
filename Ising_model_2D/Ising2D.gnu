#set terminal qt
N=100

set xrange [0:N]
set yrange [0:N]

#set output "test.png"

# set title "Testowo 10x10"
# do splotu 
unset surface
unset colorbox
unset key
set tic scale 0

set view map
set size square

#color palette
set palette maxcolors 2
#set palette defined (0 "royalblue", 1 "red")
set palette defined (0 "black", 1 "white")

# fname="100x100_after500_beta_05.txt"
fname = "wyniki/zad2_T0.2.txt"

plot fname using ($1 +0.5):($2+0.5):3 with image notitle, \
 # fname using ($1 +0.5):($2+0.5):($3 == 1 ? "\U+2191" : "\U+2193" ) with labels font "serif,10" notitle

