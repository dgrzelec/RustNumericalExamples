set terminal gif animate nooptimize delay 250 size 500,500
set output "results/grains.gif"

N=10
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
set palette maxcolors 5
set palette defined (0 "black", 1 "green", 2 "purple", 3 "yellow", 4 "red")


# loop
# you need to manually insert number of frames generated from code
do for [ii=0:8] {

    fname = sprintf('results/frame_%02d.txt', ii)
    plot fname using ($1 -0.5):($2-0.5):($3 <= 3 ? $3 : 4) with image notitle, \
    fname using ($1 -0.5):($2-0.5):($3 == 0 ? '' : $3) with labels font "serif,10" notitle


}
set output
