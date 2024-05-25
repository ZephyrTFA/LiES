@echo off

@REM Red color
color 04
if not exist .env (
    echo .env file does not exist.
    exit /b 1
)

for /f "delims== tokens=1,2" %%a in ('findstr /b GAME_DIR .env') do (
    if "%%a"=="GAME_DIR" (
        set GAME_DIR=%%b
    )
    if "%%a"=="LIES_PARSE_LOG_MODE" (
        set LIES_PARSE_LOG_MODE=%%b
    )
    if "%%a"=="DME_FILE" (
        set LIES_PARSE_LOG_FILE=%%b
    )
)

if not defined GAME_DIR (
    echo GAME_DIR is not set
    exit /b 1
)

if not defined LIES_PARSE_LOG_MODE (
    set LIES_PARSE_LOG_MODE=0
)

if not defined DME_FILE (
    echo DME_FILE is not set
)

@REM Blue color
color 01

echo Building docker image with volume directory: %GAME_DIR%
docker build -q -t lies .

docker ps -a --filter "name=LiES" | findstr /v "CONTAINER ID" >nul
if errorlevel 0 (
    echo Removing old container...
    docker rm -f LiES
)

echo Starting docker...
color
docker run --rm --tty --name LiES -e GAME_DIR=/usr/src/ss13 -v %GAME_DIR%:/usr/src/ss13 -p 4000:80 lies

echo Cleaning up docker images...
docker image prune -f >nul
