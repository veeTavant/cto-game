trait Work {
        fn work(&self);
}

trait Learn {
    fn learn(&self);
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
    fn work(&self) {
        println!("working!");
    }
}

/*
struct Developer {
    is_a_parrot: bool
}
*/

/*
impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}
 */