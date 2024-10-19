#!/bin/bash

# Check if an argument was provided
if [ -z "$1" ]; then
  echo "Please provide an argument."
  exit 1
fi

# Set the current script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Search for the plantuml*.jar file in the current directory
JAR_FILE=$(find "$SCRIPT_DIR" -maxdepth 1 -name "plantuml*.jar" | head -n 1)

# Check if the JAR file was found
if [ -z "$JAR_FILE" ]; then
  echo "No plantuml*.jar file found in the current folder."
  exit 1
fi

# Run the Java program with the found JAR file and the provided argument
java -Dfile.encoding=UTF-8 -jar "$JAR_FILE" -tsvg -charset UTF-8 "$1"

# Exit the script
exit 0
