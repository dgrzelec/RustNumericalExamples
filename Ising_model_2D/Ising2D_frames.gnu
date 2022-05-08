set terminal gif animate nooptimize delay 10 size 750,750
set output "500x500_highT.gif"

N=500

set xrange [0:N]
set yrange [0:N]

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
# fname = "wyniki/zad2_T0.2.txt"


do for [i = 0:99:1] {
        
    fname = sprintf("frames/%04d.txt", i)
    plot fname using ($1 +0.5):($2+0.5):3 with image notitle, \

} 
set output
