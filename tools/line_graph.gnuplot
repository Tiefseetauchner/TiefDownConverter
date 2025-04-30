set datafile separator ","
set xdata time
set timefmt "%Y-%m-%d %H:%M"
set format x "%Y-%m-%d"
set terminal pngcairo size 2400,1800 enhanced font "Cormorant,24"
set output outputfile

set title filetype." LOC Over Time"
set xlabel "Date"
set ylabel "Lines of Code"
set grid
set key off
set xtics rotate by -45

plot datafile using 1:2 with lines lw 2 lc rgb "blue"