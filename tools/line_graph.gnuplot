set datafile separator ","
set xdata time
set timefmt "%Y-%m-%d"
set format x "%Y-%m-%d"
set terminal pngcairo size 1200,600 enhanced font "Cormorant,10"
set output outputfile

set title "Rust LOC Over Time"
set xlabel "Date"
set ylabel "Lines of Code"
set grid
set key off
set xtics rotate by -45

plot datafile using 1:2 with lines lw 2 lc rgb "blue"