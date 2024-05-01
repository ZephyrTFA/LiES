@echo off

if not exist .env (
    echo .env file does not exist.
    exit /b 1
)

for /f "delims== tokens=1,2" %%a in ('findstr /b GAME_DIR .env') do (
    if "%%a"=="GAME_DIR" (
        set GAME_DIR=%%b
    )
)

if not defined GAME_DIR (
    echo GAME_DIR is not set
    exit /b 1
)


echo Building docker image with volume directory: %GAME_DIR%
docker build -q -t lies .

docker ps -a --filter "name=LiES" | findstr /v "CONTAINER ID" >nul
if errorlevel 1 (
    echo Starting docker...
    docker run --tty --name LiES -e GAME_DIR=/usr/src/ss13 -v %GAME_DIR%:/usr/src/ss13 -p 4000:80 lies
) else (
    echo Restarting docker...
    docker restart LiES >nul
    echo Running LiES...
    docker exec LiES /bin/sh -c "./start.sh"
)
