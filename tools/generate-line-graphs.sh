#!/bin/bash

script_dir=$(dirname "$0")

commit_file="commit_history.csv"
data_file="line_graph_data.csv"

BRANCH="main"

while getopts "eob:" opt; do
    case $opt in
        e)
            EXTENSION=$OPTARG
            ;;
        o)
            OUTPUT_FILE=$OPTARG
            ;;
        b)
            BRANCH=$OPTARG
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
    esac
done

git stash
git checkout $BRANCH
git pull

echo "Collecting commit history..."
git log --pretty=format:"%h,%ad" --date=short > "$commit_file"

echo "date,loc" > "$data_file"

while IFS=',' read -r commit date; do
    printf "\rProcessing commit %s...%-20s" "$commit" ""
    git checkout $commit > /dev/null 2>&1

    loc=$(find . -name '*.rs' -type f -not -path "./target/*" -exec cat {} + | wc -l)

    echo "$date,$loc" >> "$data_file"
done < "$commit_file"


echo "Generating line graph..."

gnuplot -e "datafile='$data_file'; outputfile='$OUTPUT_FILE'" "$script_dir/line_graph.gnuplot"

echo "Cleaning up..."
rm "$commit_file"
rm "$data_file"

echo "Done!"

git checkout $BRANCH
git stash pop