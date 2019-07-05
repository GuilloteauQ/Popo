#! /bin/bash
#
# Dependencies:
#
# 	- pandoc (install following http://tutorialspots.com/how-to-install-pandoc-on-centos-4902.html)
#

CC=pandoc
ROOT_FOLDER=$(pwd)
SRC_FOLDER=${ROOT_FOLDER}/src/
OUTPUT_FOLDER=${ROOT_FOLDER}/doc/
INPUT_FORMAT="md"
OUTPUT_FORMAT="html"
CSS_FOLDER=${ROOT_FOLDER}/css
CSS_FILE=${CSS_FOLDER}/pandoc.css

echo $CSS_FILE

# Making the Folder for the documentation to be generated in
mkdir ${OUTPUT_FOLDER}

# Generating a .html file for every Markdown file in the SRC_FOLDER
for md_file in ${SRC_FOLDER}*.${INPUT_FORMAT}
do
    md_file_base_name=$(basename $md_file .${INPUT_FORMAT})
    output_file=${OUTPUT_FOLDER}${md_file_base_name}.${OUTPUT_FORMAT}
    ${CC} -s --from gfm --to html5 --css ${CSS_FILE} $md_file -o ${output_file} --metadata pagetitle="Daily Report"

    # Replace the '[ ]' with a checkbox manually !

    sed --in-place 's/\[ \]/<input type="checkbox" disabled="" \/>/g' ${output_file}
    sed --in-place 's/\[x\]/<input type="checkbox" checked="" \/>/g' ${output_file}
    sed --in-place 's/\[X\]/<input type="checkbox" checked="" \/>/g' ${output_file}
done

# Creates a index.html page to easily go to a report
PAGE_NAME="index.html"
PAGE=${ROOT_FOLDER}/${PAGE_NAME}
TITLE="Daily Reports"
H1_TITLE=$TITLE

## Writing HTML Header
echo "<html><head><title>${TITLE}</title></head><body><h1 align='center'>${H1_TITLE}</h1>" > ${PAGE}

## Writing the list of link

echo "<ul>" >> ${PAGE}

for html_file in ${OUTPUT_FOLDER}*.${OUTPUT_FORMAT}
do
    echo "<li> <a href='$html_file'>$(basename $html_file .${OUTPUT_FORMAT})</a>" >> ${PAGE}
done

echo "</ul>" >> ${PAGE}

## Writing HTML Footer
echo "</body></html>" >> ${PAGE}
