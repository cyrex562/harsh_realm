# PowerShell script to set up virtual environment and run Excel to CSV converter

Write-Host "Setting up Python virtual environment and running Excel to CSV converter..." -ForegroundColor Green

# Get the project root directory (parent of scripts directory)
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

# Change to project root
Set-Location $ProjectRoot

# Check if virtual environment exists
if (-not (Test-Path "venv")) {
    Write-Host "Creating virtual environment..." -ForegroundColor Yellow
    python -m venv venv
}

# Activate virtual environment
Write-Host "Activating virtual environment..." -ForegroundColor Yellow
& "venv\Scripts\Activate.ps1"

# Install dependencies
Write-Host "Installing dependencies..." -ForegroundColor Yellow
pip install -r requirements.txt

# Run the converter script
Write-Host "Running Excel to CSV converter..." -ForegroundColor Yellow
python scripts\excel_to_csv_converter.py

# Deactivate virtual environment
deactivate

Write-Host "Done!" -ForegroundColor Green
Read-Host "Press Enter to continue" 