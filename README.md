<div>


# Lab Kit 🚀


</div>
<div style="text-align: center; background-color: #F1EFE7;">
    <img src="Assets/images/thmb.svg">
</div>
<br>
<div style="text-align: center;">
<img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT">
<!-- <img src=""> -->
</div>

## Overview

The **Lab Kit** is a command-line interface (CLI) tool designed for IUT students to simplify the process of organizing and completing lab tasks. It automatically creates structured folders and generates C files based on the student's ID, lab number, and the number of tasks required.

In addition, the Lab Kit includes a **Test Runner** feature, which enables you to save test cases and run them to validate your code against specific requirements.

---

## Features ✨

- **Intuitive User Input**: The tool provides clear instructions throughout the process, so you can easily get started—even if you forget how it works.
- **C File Generation**: Automatically generates C files for each task, adhering to a consistent naming convention and including boilerplate code to help you get started quickly.
- **Test Runner**: A user-friendly tool that validates your code by running predefined test cases to ensure it works as expected.

---

## Installation ⚙️

1. Visit the [Releases](https://github.com/notenderdreams/IUT-Lab-Kit/releases/tag/cli) page on GitHub.
2. Download the `Lab-Kit-setup.exe` file.
3. Double-click the installer to begin installation.
4. Follow the on-screen instructions to complete the setup.

Once installed, you’re all set to start using the Lab Kit!

---

## How to Use 📋

### Initialize the Project: `init`

To get started, open your terminal and run the following command:

```bash
lab init
```

This command will prompt you to enter:
1. **Student ID**
2. **Lab Number**
3. **Number of Tasks**

Alternatively, you can provide these details directly in the command:

```bash
lab init [studentID] [labNo] [taskCount]
```

**Example:**

```bash
lab init 230041299 11 4
```

This will generate the following files:

- `230041299_Lab11_Task1.c`
- `230041299_Lab11_Task2.c`
- `230041299_Lab11_Task3.c`
- `230041299_Lab11_Task4.c`

Additionally, a `.lab/config.json` file will be created to store configuration settings.

---

### Set Test Cases: `set`

To add test cases for a specific task, run:

```bash
lab set [taskNumber]
```

**Example:**

```bash
lab set 1
```

This will open a menu displaying current test cases:

```
Current test cases:
1. Input: 5 2, Output: 10

Commands:
a - Add test case
d - Delete test case
r - Run tests
q - Save and quit
```

- Press **a** to add a new test case.
- Press **d** to delete a test case.
- Press **r** to run the test cases for that task from the set menu.
- Press **q** to save and quit the test case setup.

>**Note:** When entering multi-line input in a test case, use `\n` for line breaks and be cautious about the spaces. Example:

For 
```
Hello 
World!
```
It will be 
```bash
Hello\nWorld!
```


---

### Run Test Cases: `run`

To execute the test cases for a specific task, run:

```bash
lab run [taskNumber]
```

**Example:**

```bash
lab run 1
```

To run test cases for **all tasks**, simply run:

```bash
lab run
```

The tool will run all the test cases across all tasks and display the results, showing how many tests passed and how many failed.

---

### Help Command: `help`

If you need any assistance or further instructions, simply run:

```bash
lab help
```

This will display a list of available commands and their descriptions.



<div style="text-align: center; margin-top: 30px; background-color: #F1EFE7; padding: 10px;">
    <p>Created with ❤️ by <a href="https://github.com/notenderdreams" target="_blank">notenderdreams</a></p>
    <!-- <p>Special thanks to all the contributors and the IUT community for their support!</p> -->
    <p>Feel free to fork, contribute, and open issues on GitHub.</p>
</div>
