#!/bin/bash

ROOT_FOLDER=/home/taitnet/Documents/API_Adaptor/DailyReports
SRC_FOLDER=${ROOT_FOLDER}/src
DOC_FOLDER=${ROOT_FOLDER}/doc
FORMAT=md
cd ${ROOT_FOLDER}

function get_last_md_file {
    NUMBER_FILES=$(ls -1q ${SRC_FOLDER} | wc -l)
    printf -v LAST_N "%02d" $NUMBER_FILES

    LAST_FILES=${SRC_FOLDER}/${LAST_N}_*.${FORMAT}
    LAST_FILE=${SRC_FOLDER}/${LAST_N}_*.${FORMAT}

    for file in ${LAST_FILES}
    do
        LAST_FILE=${file}
    done

    # ${YESTERDAY_FILE}
}

action=$1
case "$action" in
    'new')
        bash ${ROOT_FOLDER}/scripts/generate_new_md_file.sh > /dev/null 2>&1
        bash ${ROOT_FOLDER}/scripts/generate_doc.sh > /dev/null 2>&1
        get_last_md_file
        vim ${LAST_FILE}
        ;;
    'show')
        bash ${ROOT_FOLDER}/scripts/generate_doc.sh > /dev/null 2>&1
        firefox ${ROOT_FOLDER}/index.html
        ;;
    'see')
        bash ${ROOT_FOLDER}/scripts/generate_doc.sh > /dev/null 2>&1
        get_last_md_file
        last_html_file=${DOC_FOLDER}/$(basename ${LAST_FILE} md)html
        firefox ${last_html_file}
        ;;
    'rm-last')
        get_last_md_file
        echo "Are you sure you want to delete '$(basename ${LAST_FILE})' ?"
        read -p "Press any key to continue"
        rm ${LAST_FILE}
        rm -rf ${DOC_FOLDER}
        bash ${ROOT_FOLDER}/scripts/generate_doc.sh > /dev/null 2>&1
        ;;
    *)
        get_last_md_file
        vim ${LAST_FILE}
        bash ${ROOT_FOLDER}/scripts/generate_doc.sh > /dev/null 2>&1
        ;;
esac
