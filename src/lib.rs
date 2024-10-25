pub static C_CODE: &str = r#"
#include <stdio.h>

int main(){


    return 0;
}
"#;


pub const USAGE: &str = r#"
Usage: lab [student_id] [lab_number] [number_of_tasks]

Arguments:
    student_id    Your student ID (e.g., 230041234)
    lab_number    The lab number (e.g., 7)
    number_of_tasks    The number of tasks to create (e.g., 4)

Example:
    lab 230041234 7 4
"#;
