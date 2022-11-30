mod team;

use crate::employee::Employee;
use std::collections::HashMap;


trait SetDirection {
    fn set_direction(&mut self, direction: CompanyDirection);
}


// https://rust-classes.com/chapter_4_3.html
//

// CompanyDirection is the strategic component of how the company is approaching software development
//
//
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CompanyDirection {
    B2B,
    B2C /* 
    SaaS,
    Whatever */
}

pub enum MarketingStrategy {
    Organic,
    Campaign,
    Direct,
    Content
}

pub enum GrowthStrategy {
    None,
    Focussed,
    Aggressive
}

pub enum Ownership {
    Private,
    Public
}

pub enum HiringStrategy {
    Passive,
    Opportunistic,
    Aggressive,
    Frozen
}

//impl PartialEq for CompanyDirection {/
//
//}

//#[derive(Debug)]
pub struct Company {
    _cash_in_bank: u32,                  // starting cash
    _direction: CompanyDirection,
    _employees: HashMap<String, Employee>,
    _ownership: Ownership,
    _growth_strategy: GrowthStrategy,
    _marketing_strategy: MarketingStrategy,
    _hiring_strategy: HiringStrategy
}

impl Company {

    pub fn new(cash_in_bank: u32, direction: CompanyDirection) -> Company {   
        return Company { _cash_in_bank: cash_in_bank,
                         _direction: direction,
                         _employees: HashMap::new(),
                         _ownership: Ownership::Private,
                         _growth_strategy: GrowthStrategy::Focussed,
                         _marketing_strategy: MarketingStrategy::Organic,
                         _hiring_strategy: HiringStrategy::Passive
                     };
    }

    // Immutable access.
    pub fn cash_in_bank(&self) -> u32 {
        self._cash_in_bank
    }
    
    pub fn direction(&self) -> CompanyDirection {
        self._direction
    }

    pub fn remove_cash(&mut self, cash :u32) {
        self._cash_in_bank -= cash;
    }

    pub fn add_cash(&mut self, cash :u32) {
        self._cash_in_bank += cash;
    }

    pub fn add_employee(&mut self, employee :Employee) {
        self._employees.insert(employee.name(), employee);
    }

    pub fn get_employees(&self) -> &HashMap<String, Employee> {
        &self._employees
    }

    // Mutable access.
//    fn first_name_mut(&mut self) -> &mut String {
//       &mut self.first_name
//   }
//   fn last_name_mut(&mut self) -> &mut String {
//       &mut self.last_name
//   }
}

impl SetDirection for Company {
    fn set_direction(&mut self, direction: CompanyDirection) {
        //Company { cash_in_bank: self.cash_in_bank, customers: self.customers, cost_of_service_per_month: self.cost_of_service_per_month, direction: direction }
        self._direction = direction;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn company_direction_test() {

        let mut company = Company::new(100, CompanyDirection::B2B);

        company.set_direction(CompanyDirection::B2C);
        assert_eq!(company.direction(), CompanyDirection::B2C);
    }

    #[test]
    fn company_cash_test() {

        let mut company = Company::new(100, CompanyDirection::B2B);

        company.remove_cash(10);
        assert_eq!(company.cash_in_bank(), 90);

        company.add_cash(10);
        assert_eq!(company.cash_in_bank(), 100);
    }

}
