@echo off
echo Setting up Python virtual environment and running Excel to CSV converter...

REM Get the project root directory (parent of scripts directory)
set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR%..

REM Change to project root
cd /d "%PROJECT_ROOT%"

REM Check if virtual environment exists
if not exist "venv" (
    echo Creating virtual environment...
    python -m venv venv
)

REM Activate virtual environment
echo Activating virtual environment...
call venv\Scripts\activate.bat

REM Install dependencies
echo Installing dependencies...
pip install -r requirements.txt

REM Run the converter script
echo Running Excel to CSV converter...
python scripts\excel_to_csv_converter.py

REM Deactivate virtual environment
call venv\Scripts\deactivate.bat

echo Done!
pause 