@echo off
setlocal

REM Print status messages
echo Running script sequence: Rust -^> Python

REM Run the Rust program using cargo run
echo Running Rust program...
cargo run
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Rust program failed with exit code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

echo Rust program completed successfully.

REM Run the Python script with the exact command you use in VS Code
echo Running Python script...
python -u "d:\Programming\sentiment-analysis\main.py"
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Python script failed with exit code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

echo Python script completed successfully.
echo Full execution sequence completed.