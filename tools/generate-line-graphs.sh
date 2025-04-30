#!/bin/bash

script_dir=$(dirname "$0")

commit_file="commit_history.csv"
data_file="line_graph_data.csv"

BRANCH="main"

info() {
    echo "Plot a line graph of the file line commit history of a git repository."

    usage
}

usage() {
    echo "Usage: $0

    -e EXTENSION: The file extension to include in the search.
    -o OUTPUT_FILE: The output file path.
    -b BRANCH: The branch to use."
}


while getopts "e:o:b:h" opt; do
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
        h)
            info
            exit 0
            ;;
        \?)
            echo "Use -h for help"
            exit 1
            ;;
    esac
done

if [ -z "$EXTENSION" ]; then
    echo "Use -e to specify the file extension"
    exit 1
fi

if [ -z "$OUTPUT_FILE" ]; then
    echo "Use -o to specify the output file"
    exit 1
fi

git stash
git checkout $BRANCH
git pull

echo "Collecting commit history..."
git log --pretty=format:"%h,%ad" --date=iso > "$commit_file"

echo "datetime,loc" > "$data_file"

while IFS=',' read -r commit date; do
    printf "\rProcessing commit %s...%-20s" "$commit" ""
    git checkout $commit > /dev/null 2>&1

    loc=$(find . -name "*$EXTENSION" -type f -not -path "./target/*" -exec cat {} + | wc -l)

    datetime=$(echo "$date" | cut -d' ' -f1-2)
    echo "$datetime,$loc" >> "$data_file"
done < "$commit_file"

git checkout $BRANCH
git stash pop

echo "Generating line graph..."

(head -n 1 "$data_file" && tail -n +2 "$data_file" | sort -k1,1) > "$data_file.sorted"
mv "$data_file.sorted" "$data_file"

gnuplot -e "datafile='$data_file'; outputfile='$OUTPUT_FILE'" "$script_dir/line_graph.gnuplot"

echo "Cleaning up..."
rm "$commit_file"
rm "$data_file"

echo "Done!"

