# Scripts Directory

This directory contains utility scripts for the Harsh Realm project.

## Excel to CSV Converter

The `excel_to_csv_converter.py` script converts the solar system data from Excel format to CSV format with custom column mappings.

### Features

- Loads `data/solar_system_data 1.xlsx`
- Exports columns A-Z (first 26 columns) to CSV
- Applies custom header mappings as specified
- Outputs to `data/solar_system_data.csv`

### Column Mappings

The script maps the following Excel headers to new CSV headers:

| Original Header | New Header |
|----------------|------------|
| Region | region |
| Body | body |
| Type | type |
| Atmo | atmosphere |
| Surface T | surface_temperature |
| AP | aphelion_apogee |
| PE | perihelion_perogee |
| SMA | semi_major_axis |
| EC | eccentricity |
| OP | orbital_period |
| MA | mean_anomaly |
| IN | inclination |
| LAN | longitude_of_ascending_node |
| APE | argument_of_perihelion |
| M | mass |
| d | diameter |
| r | radius |
| c | circumference |
| sg | surface_gravity |
| ev | escape_velocity |
| rp | rotational_period |
| at | axial_tilt |
| th | total_hexes |
| he | hexes_at_equator |
| hp | hexes_at_poles |
| lb | latitude_bands |

### Running the Script

#### Option 1: Using the batch file (Windows)
```cmd
scripts\run_converter.bat
```

#### Option 2: Using the PowerShell script (Windows)
```powershell
scripts\run_converter.ps1
```

#### Option 3: Manual setup and run
```bash
# Create virtual environment
python -m venv venv

# Activate virtual environment
# Windows:
venv\Scripts\activate
# Linux/Mac:
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Run the script
python scripts/excel_to_csv_converter.py
```

### Dependencies

- Python 3.8+
- pandas >= 2.0.0
- openpyxl >= 3.1.0

### Output

The script will create `data/solar_system_data.csv` with the converted data and mapped column headers. 