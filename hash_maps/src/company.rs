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

    fn count_persons_from_department(&self, department: &String) -> usize {
        let persons_from_department = self.departments.get(department);
        match persons_from_department {
            Some(names) => names.len(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_employee() {
        let mut company = Company::new();
        let department = "myDepartment";;
        _create_and_add_employee_to_department(&mut company, department.to_string());

        let num_of_persons = company.count_persons_from_department(&department.to_string());
        assert_eq!(num_of_persons, 1);

        _create_and_add_employee_to_department(&mut company, department.to_string());
        let num_of_persons = company.count_persons_from_department(&department.to_string());
        assert_eq!(num_of_persons, 2);
    }

    fn _create_and_add_employee_to_department(company: &mut Company, department_name: String) {
        let employee = EmployeeInfo::new(String::from("myName"), department_name);
        company.add_employee(&employee);
    }
}
