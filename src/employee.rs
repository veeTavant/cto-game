// Structure
//
//                            CEO
//                             |
//      CTO            CFO            CPO            CMO
//       |              |              |              |
//    Developer     Accountant   Product Owner     Marketeer
//     Tester       Salesperson  Product Manager
//  Administrator 
//
//
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum EmployeeType {
    Developer,
    Tester,
    Salesperson,
    Marketeer,
    Administrator,
    FinanceDirector,
    CEO,
    CTO,
    CMO,
    CPO,
    Accountant,
    ProductOwner,
    ProductManager
}


pub struct Employee {
     _employee_type: EmployeeType,
     _id: u16,
     _name: String,
     _age: u16,
     _compensation: u16,
     _efficiency: u16,
     _talent: u16
}

impl Employee {

    pub fn new(employee_type :EmployeeType, id :u16, name :String, age :u16, compensation :u16, efficiency :u16, talent :u16) -> Employee {
        Employee { _employee_type: employee_type, _id: id, _name: name, _age: age, _compensation: compensation, _efficiency: efficiency, _talent: talent }
    }  

    pub fn employee_type(&self) -> EmployeeType {
        self._employee_type
    }

    pub fn id(&self) -> u16 {
        self._id
    }

    pub fn name(&self) -> String {
        self._name.to_string()
    }

    pub fn age(&self) -> u16 {
        self._age
    }

    pub fn salary(&self) -> u16 {
        self._compensation
    }

    pub fn efficiency(&self) -> u16 {
        self._efficiency
    }

    pub fn talent(&self) -> u16 {
        self._talent
    }

    pub fn add_talent(&mut self, talent :u16) {
        self._talent += talent
    }

    pub fn add_efficiency(&mut self, efficiency :u16) {
        self._efficiency += efficiency
    }

    pub fn remove_efficiency(&mut self, efficiency :u16) {
        self._efficiency -= efficiency
    }

    pub fn add_compensation(&mut self, compensation :u16) {
        self._compensation += compensation
    }

    pub fn remove_compensation(&mut self, compensation :u16) {
        self._compensation -= compensation
    }

    pub fn add_age(&mut self, age :u16) {
        self._age += age
    }

    pub fn set_employee_type(&mut self, employee_type :EmployeeType) {
        self._employee_type = employee_type
    }

}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn employee_age_test() {
        let mut employee = Employee::new(EmployeeType::Developer, 1, "Developer 2".to_string(), 23, 35, 89, 77);
        employee.add_age(11);
        assert_eq!(employee._age, 34);
    }

    #[test]
    fn employee_efficiency_tests() {
        let mut employee = Employee::new(EmployeeType::Developer, 2, "Developer 2".to_string(), 23, 35, 89, 77);
        
        employee.add_efficiency(11);
        assert_eq!(employee._efficiency, 100);

        employee.remove_efficiency(11);
        assert_eq!(employee._efficiency, 89);
    }

    #[test]
    fn employee_compensation_test() {
        let mut employee = Employee::new(EmployeeType::Developer, 3, "Developer 2".to_string(), 23, 35, 89, 77);
        employee.add_compensation(10);
        assert_eq!(employee._compensation, 45);

        employee.remove_compensation(10);
        assert_eq!(employee._compensation, 35)
    }

    #[test]
    fn employee_type_test() {
        let mut employee = Employee::new(EmployeeType::Developer, 4, "Developer 2".to_string(), 23, 35, 89, 77);
        assert_eq!(employee._employee_type, EmployeeType::Developer);

        employee.set_employee_type(EmployeeType::Administrator);
        assert_eq!(employee._employee_type, EmployeeType::Administrator);
    }

    #[test]
    fn employee_id_test() {
        let employee = Employee::new(EmployeeType::Developer, 5, "Developer 2".to_string(), 23, 35, 89, 77);
        assert_eq!(employee.id(), 5);
    }


}