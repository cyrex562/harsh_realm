#!/usr/bin/env python3
import re
import sys
import subprocess
from pathlib import Path

def get_current_version():
    cargo_toml = Path("Cargo.toml")
    if not cargo_toml.exists():
        raise FileNotFoundError("Cargo.toml not found")

    with open(cargo_toml, 'r') as f:
        content = f.read()
        match = re.search(r'version = "(\d+\.\d+\.\d+)"', content)
        if match:
            return match.group(1)
        raise ValueError("Version not found in Cargo.toml")

def update_version(version_type):
    current_version = get_current_version()
    major, minor, patch = map(int, current_version.split('.'))

    if version_type == 'major':
        new_version = f"{major + 1}.0.0"
    elif version_type == 'minor':
        new_version = f"{major}.{minor + 1}.0"
    elif version_type == 'patch':
        new_version = f"{major}.{minor}.{patch + 1}"
    else:
        raise ValueError("Invalid version type. Use 'major', 'minor', or 'patch'")

    # Update Cargo.toml
    cargo_toml = Path("Cargo.toml")
    with open(cargo_toml, 'r') as f:
        content = f.read()

    new_content = re.sub(
        r'version = "\d+\.\d+\.\d+"',
        f'version = "{new_version}"',
        content
    )

    with open(cargo_toml, 'w') as f:
        f.write(new_content)

    # Create a commit with the version update
    subprocess.run(['git', 'add', 'Cargo.toml'])
    subprocess.run(['git', 'commit', '-m', f'Bump version to {new_version}'])

    print(f"Version updated to {new_version}")
    return new_version

def auto_bump_patch():
    """Automatically bump patch version for each push"""
    try:
        update_version('patch')
    except Exception as e:
        print(f"Error auto-bumping version: {e}")
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python version_manager.py [major|minor|patch|auto]")
        sys.exit(1)

    version_type = sys.argv[1].lower()
    if version_type == 'auto':
        auto_bump_patch()
    else:
        update_version(version_type)