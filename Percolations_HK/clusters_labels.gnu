N=16

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
set palette defined (0 "white", 1 "gray")


fname = "Label_p_0.8.txt"

plot fname using ($1 -0.5):($2-0.5):($3 == 0 ? 0 : 1) with image notitle, \
 fname using ($1 -0.5):($2-0.5):($3 == 0 ? '' : $3) with labels font "serif,10" notitle

