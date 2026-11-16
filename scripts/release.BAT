@echo off
set /p VERSION="Enter version (e.g., 0.10.3): "
echo Processing version %VERSION%...

:: Example of appending the variable to command:
powershell -ExecutionPolicy Bypass -File scripts\release.ps1  %VERSION%
