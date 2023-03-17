use crate::employee::{Employee, EmployeeType};
use std::collections::HashMap;
use std::fmt;
//use super::Software;


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

impl fmt::Display for CompanyDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum MarketingStrategy {
    Organic /* ,
    Campaign,
    Direct,
    Content*/
}

pub enum GrowthStrategy {
    Focussed
/*  None,
    Aggressive*/
}

pub enum Ownership {
    Private /* ,
    Public */
}

pub enum HiringStrategy {
    Passive /* ,
    Opportunistic,
    Aggressive,
    Frozen */
}


//#[derive(Debug)]
//#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

    // Do we need to zero out or instead just go negative?
    //
    pub fn remove_cash(&mut self, cash :u32) {

        if cash < self._cash_in_bank {
            self._cash_in_bank -= cash;
        } else {
            self._cash_in_bank = 0;
        }

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

    pub fn get_number_of_employees(&self, employee_type: EmployeeType) -> u16 {
        let mut employees = 0;
        for (_key, val) in self._employees.iter() {
            if val.employee_type() == employee_type {
                employees += 1;
            }
        }

        return employees;
    }

    // Queue up the payroll for execution - how do we work out bonuses and year end type stuff?
    // Also when do we work out performance raises etc?
    //
    pub fn queue_payroll(&mut self) -> bool {
        // In the simple case we just execute it
        //
        let mut payroll_amount :u32 = 0;

        // First add it up
        for (_key, val) in self._employees.iter() {
            payroll_amount += val.salary()
        }

        if payroll_amount > self.cash_in_bank() {
            self._cash_in_bank = 0;
            return false;
        }

        self._cash_in_bank -= payroll_amount;
        return true;
    }

    // What is our development capacity?
    //
    // This is a function of developers, reliability, 
    pub fn get_development_capacity(&self, reliability: u16, quality: u16) -> u16 {

        // Count developers and their levels
        //

        // https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter
        let mut developer_skills = 0;
        let mut developers = 0;


        for (_key, val) in self._employees.iter() {
            if val.employee_type() == EmployeeType::Developer {
                developers += 1;
                developer_skills += val.efficiency();        
            }
        };

        if developers > 0 {
            developer_skills = ( developer_skills as f32 / developers as f32 ) as u16; 
        }

        // Got number of developers and combined average skills
        //
        if quality < 50 {
            developer_skills /= 2;
        }

        // Adjust by reliability
        //
        if reliability < 30 {
            developer_skills /= 4;
        } else if reliability < 70 {
            developer_skills /= 2;
        }

        return developer_skills;
            
    }

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
