# PlantUML Wrapper

A simple wrapper script to customize the usage of PlantUML.
- Exports PlantUML diagrams in **SVG format**.
- Ensures **UTF-8** encoding during diagram generation.

## Installation

### 0. Prerequisites
**Java** must be installed on your system.  
You can check it by executing `java -version` in the terminal.

### 1. Create a User Executables Folder
To make the script callable from terminal, create a folder for your custom scripts and executables:

#### Windows
1. Create a folder like `C:\Users\YourUsername\bin`.
2. Add it to your system’s `PATH`:
   - Open `Environment Variables`.
   - Edit the `Path` variable and add the folder path.

#### Linux & MacOS
1. Create a folder, e.g., `~/bin`.
2. Add it to your `PATH`:
   ```bash
   echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc  # For Linux
   echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc   # For MacOS or zsh users
   source ~/.bashrc   # or ~/.zshrc
   ```

### 2. Install the Script
1. Download the latest plantuml jar file from [the official PlantUML website](http://plantuml.com/download).
2. Put it in the user executables folder created above.
3. Place `plantuml.bat` (Windows) or `plantuml.sh` (Linux and MacOS) in the user executables folder created above.
4. Ensure the script is executable (only for Linux and MacOS):
   ```bash
   chmod +x ~/bin/plantuml.sh
   ```

## Usage
Now you can call the script from the terminal:
```bash
plantuml diagram.puml
```
