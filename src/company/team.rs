use chrono::{DateTime};
use chrono::{Local};

use super::Employee;

pub enum TeamType {
    Agile,
    Platform,
    Etc
}


pub struct Team {
    _name: String,
    _id: u16,
    _members: Vec<Employee>,
    _created: DateTime<Local>,
    _updated: DateTime<Local>,
    _productivity: u16   // 0 - 100
}

impl Team {
    pub fn new(name: String, id: u16, members: Vec<Employee>, productivity: u16) -> Team {
        Team { _name: name, _id: id, _members: members, _created: Local::now(), _updated: Local::now(), _productivity: productivity }
    }

    pub fn get_size(&self) -> usize {
         self._members.len()
    }
}

#[cfg(test)]
mod test {

    use crate::employee::EmployeeType;
    use super::Employee;
    use super::*;

    #[test]
    fn team_test() {

        let employee = Employee::new(EmployeeType::Developer, 1, "Richard".to_string(),  1, 35, 23, 35);
        let mut employee_list = Vec::new();
        employee_list.push(employee);
        let team = Team::new("Thundercats".to_string(), 0, employee_list, 50);
        assert_eq!(team.get_size(), 1);
 
    }
 
} 