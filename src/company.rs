
trait New {
    fn new(&mut self, cash: u32, customers: u32, cost: u32, direction: CompanyDirection) -> Company;
}

trait SetDirection {
    fn set_direction(&mut self, direction: CompanyDirection) -> Company;
}


// https://rust-classes.com/chapter_4_3.html
//

// Company constraints
//
#[derive(Debug, PartialEq, Eq)]
pub enum CompanyDirection {
    B2B,
    B2C /* 
    SaaS,
    Whatever */
}

//#[derive(Debug)]
pub struct Company {
    pub cash_in_bank: u32,                  // starting cash
    pub customers: u32,                     // #
    pub cost_of_service_per_month: u32,     // credits
    pub direction: CompanyDirection
}

impl New for Company {
    fn new(&mut self, cash: u32, customers: u32, cost: u32, direction: CompanyDirection) -> Company {
        Company { cash_in_bank: cash, customers: customers, cost_of_service_per_month: cost, direction: direction }
    }
}

impl SetDirection for Company {
    fn set_direction(&mut self, direction: CompanyDirection) -> Company {
        Company { cash_in_bank: self.cash_in_bank, customers: self.customers, cost_of_service_per_month: self.cost_of_service_per_month, direction: direction }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn company_tests() {

        let mut company = Company {
            cash_in_bank: 100,
            customers: 100,
            cost_of_service_per_month: 100,
            direction: CompanyDirection::B2B
          };

        let company2 = company.set_direction(CompanyDirection::B2C);

        assert_eq!(company2.direction, CompanyDirection::B2C);
    }
}