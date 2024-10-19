@echo off
REM Check if an argument was provided
if "%1"=="" (
    echo Please provide an argument.
    exit /b 1
)

REM Set the current script directory
set "SCRIPT_DIR=%~dp0"

REM Search for the plantuml*.jar file in the current directory
for %%f in ("%SCRIPT_DIR%plantuml*.jar") do (
    set "JAR_FILE=%%f"
)

REM Check if the JAR file was found
if "%JAR_FILE%"=="" (
    echo No plantuml*.jar file found in the current folder.
    exit /b 1
)

REM Run the Java program with the found JAR file and the provided argument
java -Dfile.encoding=UTF-8 -jar "%JAR_FILE%" -tsvg -charset UTF-8 %1

REM Exit the script
exit /b 0
