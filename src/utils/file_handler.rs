use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::models::question::Question;

pub fn create_task_files(student_id: &str, lab_number: u32, num_tasks: u32) -> std::io::Result<()> {
    for task_num in 1..=num_tasks {
        let filename = format!("{}_Lab{}_Task{}.c", student_id, lab_number, task_num);
        let mut file = File::create(Path::new(&filename))?;

        let template = format!(
            "#include <stdio.h>\n\n\
             int main() {{\n\
             \t// Your code here\n\
             \treturn 0;\n\
             }}\n"
        );

        file.write_all(template.as_bytes())?;
    }

    create_config_file(student_id, lab_number, num_tasks)?;
    Ok(())
}

pub fn create_config_file(
    student_id: &str,
    lab_number: u32,
    num_tasks: u32,
) -> std::io::Result<()> {
    fs::create_dir_all(".lab")?;

    let questions: Vec<Question> = (1..=num_tasks)
        .map(|i| Question {
            file_id: format!("{}_Lab{}_Task{}.c", student_id, lab_number, i),
            io: Vec::new(),
        })
        .collect();

    let config_path = Path::new(".lab").join("config.json");
    fs::write(
        config_path,
        serde_json::to_string_pretty(&questions).unwrap(),
    )?;

    Ok(())
}

