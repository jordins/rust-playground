use std::collections::HashMap;
use std::io;

fn main() {
    struct EmployeeInfo {
        name: String,
        department: String,
    };

    let mut company = HashMap::new();
    loop {
        print_help();
        let cmd_line = read_cmd_line();
        if cmd_line.len() < 2 {
            if cmd_line[0] == "list" {
                println!("Which department would you like to list?");
                let cmd_line = read_cmd_line();
                list_persons_from_department(&cmd_line[0], &company);
            } else {
                panic!("Can't understand command")
            }
        } else if cmd_line.len() == 3 {
            panic!("Provide 4 words at least with following formal: Add X to Y")
        } else {
            let employee_info = EmployeeInfo {
                name: cmd_line[1].to_string(),
                department: cmd_line[3].to_string(),
            };

            add_employee_to_company(&employee_info, &mut company);
        }
    }

    fn add_employee_to_company(
        employee: &EmployeeInfo,
        company: &mut HashMap<String, Vec<String>>,
    ) {
        company
            .entry(employee.department.to_string())
            .or_insert(Vec::new())
            .push(employee.name.to_string());

        println!(
            "Added employee {} to department {}",
            employee.name, employee.department
        )
    }

    fn list_persons_from_department(department: &String, company: &HashMap<String, Vec<String>>) {
        println!("List of persons from department {}:", department);
        let persons_from_department = company.get(department);
        match persons_from_department {
            Some(names) => {
                for name in names {
                    println!("->{}", name);
                }
            }
            None => println!("Sorry, the department {} is empty", department),
        }
    }

    fn read_cmd_line() -> Vec<String> {
        let mut employee_line = String::new();
        io::stdin()
            .read_line(&mut employee_line)
            .expect("Failed to read line");

        let vec: Vec<String> = String::from(employee_line.trim())
            .split(" ")
            .map(|word| String::from(word))
            .collect();

        vec
    }

    fn print_help() {
        println!("Write 'Add EMPLOYEE_NAME to DEPARTMENT' or the command 'list'");
    }
}
