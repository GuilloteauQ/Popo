#! /bin/bash

ROOT_FOLDER=$(pwd)
SRC_FOLDER=${ROOT_FOLDER}/src
HTML_FOLDER=${ROOT_FOLDER}/doc
FORMAT="md"

# Get the current number of files
NUMBER_FILES=$(ls -1q ${SRC_FOLDER} | wc -l)
N=$(($NUMBER_FILES + 1))
printf -v M "%02d" $N

# Get the current_date
FORMATED_DATE=$(date +"%d%m%Y")

# Get the new filename
NEW_FILE=${SRC_FOLDER}/${M}_${FORMATED_DATE}.${FORMAT}

# Creating the content for the file

echo "# Daily Report #${N}: $(date +"%d/%m/%Y")" > ${NEW_FILE}
echo "" >> ${NEW_FILE}
echo "## Goals of the day" >> ${NEW_FILE}

# We need to write the goals from yesterday that have not been done

printf -v YESTERDAY_N "%02d" $NUMBER_FILES

YESTERDAY_FILES=${SRC_FOLDER}/${YESTERDAY_N}_*.${FORMAT}
YESTERDAY_FILE=${SRC_FOLDER}/${YESTERDAY_N}_*.${FORMAT}
EMPTY_TASK="- [ ]"

for file in ${YESTERDAY_FILES}
do
    while IFS= read -r line
    do
        TASK_STATUS=${line:0:5}
        if [ "$TASK_STATUS" == "$EMPTY_TASK" ]
        then
            echo "${line}" >> ${NEW_FILE}
            echo "" >> ${NEW_FILE}
        fi

    done < "${file}"
    YESTERDAY_FILE=${file}
done

echo ${YESTERDAY_FILE}

echo "## TODO Tomorrow" >> ${NEW_FILE}
echo "## Done Today" >> ${NEW_FILE}
echo "[Previous day](${HTML_FOLDER}/$(basename ${YESTERDAY_FILE} .md).html)" >> ${NEW_FILE}


TOMORROW_FILES=${SRC_FOLDER}/${TOMORROW_N}_*.${FORMAT}
TOMORROW_FILE=${SRC_FOLDER}/${TOMORROW_N}_*.${FORMAT}

for file in ${TOMORROW_FILES}
do
    TOMORROW_FILE=${file}
done

echo "[Next day](${HTML_FOLDER}/$(basename ${TOMORROW_FILE} .md).html)" >> ${NEW_FILE}

