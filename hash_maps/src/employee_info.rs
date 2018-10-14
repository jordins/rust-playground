pub struct EmployeeInfo {
    name: String,
    department: String,
}

impl EmployeeInfo {
    pub fn new(name: String, department: String) -> EmployeeInfo {
        EmployeeInfo {
            name: name,
            department: department,
        }
    }
    pub fn get_department(&self) -> String {
        self.department.to_string()
    }
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}
