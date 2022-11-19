
// https://rust-classes.com/chapter_4_3.html
//

// Company constraints
//
#[derive(Debug)]
pub enum CompanyDirection {
    B2B,
    B2C,
    SaaS,
    Whatever
}

trait New {
    fn new(cash: u32, customers: u32, cost: u32, direction: CompanyDirection) -> Company;
}

trait SetDirection {
    fn setDirection(company: Company, direction: CompanyDirection) -> Company;
}

#[derive(Debug)]
pub struct Company {
    pub cash_in_bank: u32,                  // starting cash
    pub customers: u32,                     // #
    pub cost_of_service_per_month: u32,     // credits
    pub direction: CompanyDirection
}

impl Company {
    //CashInBank!() -> u32 { return cash_in_bank; }
}

 
impl New for Company {
    fn new(cash: u32, customers: u32, cost: u32, direction: CompanyDirection) -> Company {
        Company { cash_in_bank: cash, customers: customers, cost_of_service_per_month: cost, direction: direction }
    }
}

impl SetDirection for Company {
    fn setDirection(company: Company, direction: CompanyDirection) -> Company {
        Company { cash_in_bank: company.cash_in_bank, customers: company.customers, cost_of_service_per_month: company.cost_of_service_per_month, direction: direction }
    }
}