pub mod company;
pub mod employee_info;
use company::Company;
use employee_info::EmployeeInfo;
use std::io;

fn main() {
    let mut company = Company::new();
    loop {
        print_help();
        let cmd_line = read_cmd_line();
        if cmd_line.len() < 2 {
            if cmd_line[0] == "list" {
                println!("Which department would you like to list?");
                let cmd_line = read_cmd_line();
                company.list_persons_from_department(&cmd_line[0]);
            } else {
                panic!("Can't understand command")
            }
        } else if cmd_line.len() == 3 {
            panic!("Provide 4 words at least with following formal: Add X to Y")
        } else {
            let employee_info = EmployeeInfo::new(cmd_line[1].to_string(), cmd_line[3].to_string());
            company.add_employee(&employee_info);
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
