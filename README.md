# IUT Lab Kit 🚀

## Overview
The **IUT Lab Kit** is a command-line interface (CLI) tool designed specifically for students at IUT. It streamlines the process of organizing lab tasks by automatically creating a structured folder and generating C files based on your student ID, lab number, and the number of tasks you need to complete. 🎓

## Features ✨
- **User-Friendly Input**: Accepts input for student ID, lab number, and the number of tasks.
- **Organized Structure**: Automatically creates a dedicated folder structure to keep your lab tasks organized. 📂
- **C File Generation**: Generates C files for each task specified, making it easy to manage your coding assignments. 💻

## How to Use 📋
Open your terminal and run the following command:

```bash
lab [student_id] [lab_number] [number_of_tasks]
```

### Arguments:
- **`student_id`**: Your student ID (e.g., `230041234`).
- **`lab_number`**: The lab number (e.g., `7`).
- **`number_of_tasks`**: The total number of tasks to create (e.g., `4`).

### Example:
```bash
lab 230041234 7 4
```

This command will create a folder for lab number 7, containing 4 C files for the specified student ID.


Or,
Simply run lab in your terminal. 
```bash
lab
```

When you run the command for the first time, it will ask for your name and student ID.
After the initial setup, you can directly create your lab files without having to re-enter your details

## Installation ⚙️
1. **Run the Installer**: Double-click the setup file to begin the installation process.
2. **Add to System Variables**: After installation, add the installation directory (`C:\Program Files (x86)\IUT Lab Kit`) to your system environment variables to access the `lab` command from anywhere in the terminal.
   - To add to system variables:
     - Right-click on "This PC" or "Computer" and select "Properties."
     - Click on "Advanced system settings."
     - In the System Properties window, click on the "Environment Variables" button.
     - In the System variables section, find and select the `Path` variable, then click "Edit."
     - Click "New" and add `C:\Program Files (x86)\IUT Lab Kit`.
     - Click "OK" to close all dialog boxes.

## License 📄
This project is licensed under the MIT License. See the `LICENSE` file for more details.
