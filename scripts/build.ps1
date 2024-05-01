if (!(Test-Path .env)) {
    Write-Host ".env file does not exist." -ForegroundColor Red
    exit 1
}

$env:GAME_DIR = (Get-Content .env | Where-Object { $_ -match 'GAME_DIR' } | ForEach-Object { $($_ -split '=')[1] })

if (!$env:GAME_DIR) {
    Write-Host "GAME_DIR is not set" -ForegroundColor Red
    exit 1
}

docker image inspect lies >$null 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Building docker image with volume directory: " -NoNewline -ForegroundColor Blue
    Write-Host "$env:GAME_DIR"
    docker build -q -t lies .
} else {
    Write-Host "Valid docker image found." -ForegroundColor Blue
}

docker ps -a --filter "name=LiES" | Select-String -NotMatch "CONTAINER ID" >$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Starting docker..." -ForegroundColor Blue
    docker run --tty --name LiES -e GAME_DIR=/usr/src/ss13 -v $env:GAME_DIR:/usr/src/ss13 -p 4000:80 lies
} else {
    Write-Host "Restarting docker..." -ForegroundColor Blue
    docker restart LiES >$null
    Write-Host "Running LiES..." -ForegroundColor Blue
    docker exec --tty LiES /bin/sh -c "./start.sh"
}
