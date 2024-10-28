# IUT Lab Kit 🚀

## Overview
The **IUT Lab Kit** is a command-line interface (CLI) tool designed specifically for students at IUT. It streamlines the process of organizing lab tasks by automatically creating a structured folder and generating C files based on your student ID, lab number, and the number of tasks you need to complete. 🎓

## Features ✨
- **User-Friendly Input**: Accepts input for student ID, lab number, and the number of tasks.
- **Organized Structure**: Automatically creates a dedicated folder structure to keep your lab tasks organized. 📂
- **C File Generation**: Generates C files for each task specified, making it easy to manage your coding assignments. 💻
---

## Installation ⚙️
- Visit the [Releases](https://github.com/notenderdreams/IUT-Lab-Kit/releases/tag/cli) page on GitHub.
- Download the `IUT-Lab-Kit-setup.exe` file.
- Double-click on  the installer.
- Follow the on-screen instructions to complete the installation.
and Done.

---
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


If you want to create all the files in the current directory then just put a "." after that .
```bash
lab 230041234 7 4 .
```
This will only create the files.



Or,
Simply run lab in your terminal. 
```bash
lab
```

When you run the command for the first time, it will ask for your name and student ID.
After the initial setup, you can directly create your lab files without having to re-enter your details

---

## 🧹 Cleanup

After compilation, several unnecessary files are generated. To remove them, simply run:

```bash
lab clean
```

This will delete all generated files, keeping only the source files intact. 🗂️

---



## License 📄
This project is licensed under the MIT License.
