trait Work {
    fn work(&mut self);
}

trait Learn {
    fn learn(&mut self);
}

trait Quit {
   fn quit(&mut self);
}

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
    Administrator
 /* FinaceDirector,
    CEO,
    CTO,
    CMO,
    CPO,
    Accountant,
    ProductOwner,
    ProductManager,*/
}


pub struct Employee {
     _employee_type: EmployeeType,
     _name: String,
     _age: u16,
     _salary: u16,
     _efficiency: u16,
     _talent: u16
}

impl Employee {

    pub fn new(employee_type :EmployeeType, name :String, age :u16, salary :u16, efficiency :u16, talent :u16) -> Employee {
        Employee { _employee_type: employee_type, _name: name, _age: age, _salary: salary, _efficiency: efficiency, _talent: talent }
    }  

    pub fn employee_type(&self) -> EmployeeType {
        self._employee_type
    }

    pub fn name(&self) -> String {
        self._name.to_string()
    }

    pub fn age(&self) -> u16 {
        self._age
    }

    pub fn salary(&self) -> u16 {
        self._salary
    }

    pub fn efficiency(&self) -> u16 {
        self._efficiency
    }

    pub fn talent(&self) -> u16 {
        self._talent
    }


}

impl Work for Employee {
    fn work(&mut self) {
        println!("working!");
    }
}

impl Learn for Employee {
    fn learn(&mut self) {
        self._efficiency = self._efficiency + 1;
    }
}

impl Quit for Employee {
    fn quit(&mut self) {
        println!("quit!");
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn employee_tests() {

        let mut employee = Employee::new(EmployeeType::Developer, "Developer 2".to_string(), 23, 35, 89, 77);
        employee.learn();

        assert_eq!(employee._efficiency, 90);
    }
}