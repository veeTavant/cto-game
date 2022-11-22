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

//impl PartialEq for CompanyDirection {/
//
//}

//#[derive(Debug)]
pub struct Company {
    _cash_in_bank: u32,                  // starting cash
    _customers: u32,                     // #
    _cost_of_service_per_month: u32,     // credits
    _direction: CompanyDirection
}

impl Company {

    pub fn new(cash_in_bank: u32, customers: u32, cost_of_service_per_month: u32, direction: CompanyDirection) -> Company {   
        return Company { _cash_in_bank: cash_in_bank, _customers: customers, _cost_of_service_per_month: cost_of_service_per_month, _direction: direction };
    }

    // Immutable access.
    pub fn cash_in_bank(&self) -> u32 {
        self._cash_in_bank
    }
    pub fn customers(&self) -> u32 {
        self._customers
    }
    pub fn cost_of_service_per_month(&self) -> u32 {
        self._cost_of_service_per_month
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

    pub fn remove_customers(&mut self, customers: u32) {
        self._customers -= customers;
    }

    pub fn add_customers(&mut self, customers: u32) {
        self._customers += customers;
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

        let mut company = Company::new(100, 100, 100, CompanyDirection::B2B);

        company.set_direction(CompanyDirection::B2C);
        assert_eq!(company.direction(), CompanyDirection::B2C);
    }

    #[test]
    fn company_cash_test() {

        let mut company = Company::new(100, 100, 100, CompanyDirection::B2B);

        company.remove_cash(10);
        assert_eq!(company.cash_in_bank(), 90);

        company.add_cash(10);
        assert_eq!(company.cash_in_bank(), 100);
    }
    
    #[test]
    fn company_customer_test() {

        let mut company = Company::new(100, 100, 100, CompanyDirection::B2B);

        company.remove_customers(10);
        assert_eq!(company.customers(), 90);

        company.add_customers(10);
        assert_eq!(company.customers(), 100);
    }

}
