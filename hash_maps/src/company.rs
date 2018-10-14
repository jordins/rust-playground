use std::collections::HashMap;
use EmployeeInfo;

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, employee: &EmployeeInfo) {
        self.departments
            .entry(employee.get_department())
            .or_insert(Vec::new())
            .push(employee.get_name());

        println!(
            "Added employee {} to department {}",
            employee.get_name(),
            employee.get_department()
        )
    }

    pub fn list_persons_from_department(&self, department: &String) {
        println!("List of persons from department {}:", department);
        let persons_from_department = self.departments.get(department);
        match persons_from_department {
            Some(names) => {
                for name in names {
                    println!("->{}", name);
                }
            }
            None => println!("Sorry, the department {} is empty", department),
        }
    }
}
