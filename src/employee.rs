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
#[derive(Debug, PartialEq, Eq)]
pub enum EmployeeType {
    Developer,
    Tester,
    ProductOwner,
    ProductManager,
    Administrator,
    CTO,
    Marketeer,
    Salesperson,
    CEO,
    FinaceDirector,
    CMO,
    CPO,
    Accountant
}


pub struct Employee {
    pub _type: EmployeeType,
    pub _name: String,
    pub _age: u16,
    pub _salary: u16,
    pub _efficiency: u16,
    pub _talent: u16
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

        let mut employee = Box::new(Employee {
            _type: EmployeeType::Developer,
            _name: "Developer 2".to_string(),
            _age: 23,
            _efficiency: 35,
            _salary: 89,
            _talent: 77
          });
        
        employee.learn();
        assert_eq!(employee._efficiency, 36);
    }
}