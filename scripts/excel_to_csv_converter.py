#!/usr/bin/env python3
"""
Excel to CSV Converter for Solar System Data

This script loads the solar_system_data_1.xlsx file and exports columns A-Z to CSV
with custom header mappings as specified.
"""

import pandas as pd
import os
import sys
from pathlib import Path

def main():
    # Define the column mapping
    column_mapping = {
        'Region': 'region',
        'Body': 'body', 
        'Type': 'type',
        'Atmo': 'atmosphere',
        'Surface T': 'surface_temperature',
        'AP': 'aphelion_apogee',
        'PE': 'perihelion_perogee',
        'SMA': 'semi_major_axis',
        'EC': 'eccentricity',
        'OP': 'orbital_period',
        'MA': 'mean_anomaly',
        'IN': 'inclination',
        'LAN': 'longitude_of_ascending_node',
        'APE': 'argument_of_perihelion',
        'M': 'mass',
        'D': 'diameter',
        'R': 'radius',
        'C': 'circumference',
        'SG': 'surface_gravity',
        'EV': 'escape_velocity',
        'RP': 'rotational_period',
        'AT': 'axial_tilt',
        'TH': 'total_hexes',
        'HE': 'hexes_at_equator',
        'HP': 'hexes_at_poles',
        'LB': 'latitude_bands'
    }
    
    # Get the project root directory (parent of scripts directory)
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    
    # Define input and output paths
    input_file = project_root / "data" / "solar_system_data 1.xlsx"
    output_file = project_root / "data" / "solar_system_data.csv"
    
    # Check if input file exists
    if not input_file.exists():
        print(f"Error: Input file not found at {input_file}")
        sys.exit(1)
    
    try:
        # Read the Excel file
        print(f"Loading Excel file: {input_file}")
        df = pd.read_excel(input_file)
        
        # Display original columns for debugging
        print(f"Original columns: {list(df.columns)}")
        print(f"Data shape: {df.shape}")
        
        # Select columns A-Z (first 26 columns)
        if len(df.columns) >= 26:
            df_selected = df.iloc[:, :26]
        else:
            print(f"Warning: File has only {len(df.columns)} columns, using all available columns")
            df_selected = df
        
        # Get the original column names (headers)
        original_headers = list(df_selected.columns)
        print(f"Selected columns: {original_headers}")
        
        # Create new headers based on mapping
        new_headers = []
        for i, original_header in enumerate(original_headers):
            if original_header in column_mapping:
                new_headers.append(column_mapping[original_header])
            else:
                # If no mapping found, keep original header
                new_headers.append(original_header)
                print(f"Warning: No mapping found for column '{original_header}', keeping original name")
        
        # Apply new headers
        df_selected.columns = new_headers
        
        # Export to CSV
        print(f"Exporting to CSV: {output_file}")
        df_selected.to_csv(output_file, index=False)
        
        print(f"Successfully converted {len(df_selected)} rows and {len(df_selected.columns)} columns")
        print(f"Output file: {output_file}")
        
        # Display final column names
        print(f"Final columns: {list(df_selected.columns)}")
        
    except Exception as e:
        print(f"Error processing file: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main() 