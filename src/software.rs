use crate::world::Timeframe::YearWeek;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Architecture {
    ProofofConcept,
    Monolith,
    Microservices,
    EventDriven
}

pub enum MonetizationModel {
    OpenSource,
    Freemium,
    FreeTier,
    Proprietary
}


pub struct Software {
    _lines_of_code:         u32,    // total
    _age_of_code:           u16,    // weeks
    _complexity_of_code:    u16,    // 0 - 100
    _feature_richness:      u16,    // 0 - 100
    _ease_of_use:           u16,    // 0 - 100
    _components:            u16,    // components 1 - 100
    _services:              u16,    // services 1 - 100
    _dependencies:          u16,    // dependencies 1 - 100
    _cost_of_service:       u16,    // price of service
    _architecture: Architecture,    // what is the predominant architecture
    _customer_satisfaction: u16,    // 0 - 100
    _customers: u16,                // number of
    _capacity_percentage_active_users: u16,             // number of active users (not same as customers)
    _percentage_free_users: u16,    // percentage of free users
    _monetization_model:    MonetizationModel,
    _releases:              u16,    // how many releases have their been
    _last_release_yearweek: YearWeek,
    _outages:               u16,    // how many outages
    _technical_debt:        u16     // 0 - 100
}


// Active users can be way more than customers depending on model (B2B or B2C etc)
//
impl Software {

    pub fn new(lines_of_code: u32, age_of_code: u16, complexity_of_code: u16, cost_of_service: u16) -> Software {   
        return Software { _lines_of_code: lines_of_code,
                          _age_of_code: age_of_code,
                          _complexity_of_code: complexity_of_code,
                          _feature_richness: 0,
                          _ease_of_use: 0,
                          _components: 0,
                          _services: 0,
                          _dependencies: 0,
                          _cost_of_service: cost_of_service,
                          _architecture: Architecture::ProofofConcept,
                          _customer_satisfaction: 0,
                          _customers: 0,
                          _capacity_percentage_active_users: 0,            // 0 - 100 percentage to capacity - but can be over capacity too
                          _percentage_free_users: 0,
                          _monetization_model: MonetizationModel::Proprietary,
                          _releases: 0,
                          _last_release_yearweek: YearWeek::new(2000, 1),
                          _outages: 0,
                          _technical_debt: 0
                        };
    }


    // Simple value which is an average of the various usability factors
    // 
    pub fn usability_factor(&self) -> u16 {
        if self._releases == 0 {
            return 0
        }

        let usability = ( self._ease_of_use + self._feature_richness ) / 2 - self._technical_debt;

        if usability > 0 {
            return usability as u16
        } else  {
            return 0
        }
    }

    // Factor of number of current customers, current users and the monetization model
    //
    pub fn market_popularity(&self, current_yearweek: YearWeek) -> u16 {

        if self._releases == 0 {
            return 0
        }

        let mut popularity = ( self._customers + self._capacity_percentage_active_users ) / 2;
        
        // Now when was the last release?
        //
        

        return popularity;
    }

    pub fn lines_of_code(&self) -> u32 {
        self._lines_of_code
    }

    pub fn releases(&self) -> u16 {
        self._releases
    }

    pub fn outages(&self) -> u16 {
        self._outages
    }

    pub fn age_of_code(&self) -> u16 {
        self._age_of_code
    }

    pub fn complexity_of_code(&self) -> u16 {
        self._complexity_of_code
    }

    pub fn cost_of_service(&self) -> u16 {
        self._cost_of_service
    }

    pub fn add_lines(&mut self, lines :u32) {
        self._lines_of_code += lines
    }

    pub fn remove_lines(&mut self, lines: u32) {
        self._lines_of_code -= lines
    }

    pub fn add_age(&mut self, age :u16) {
        self._age_of_code += age
    }

    pub fn add_complexity(&mut self, complexity :u16) {
        self._complexity_of_code += complexity
    }

    pub fn remove_complexity(&mut self, complexity :u16) {
        self._complexity_of_code -= complexity
    }

    pub fn get_architecture(&self) -> Architecture {
        self._architecture
    }

    pub fn customers(&self) -> u16 {
        self._customers
    }

    pub fn capacity_percentage_active_users(&self) -> u16 {
        self._capacity_percentage_active_users
    }

    pub fn set_capacity_percentage_active_users(&mut self, users: u16) {
        self._capacity_percentage_active_users = users
    }

}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn software_lines_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_lines(100);
        assert_eq!(software.lines_of_code(), 100);

        software.remove_lines(50);
        assert_eq!(software.lines_of_code(), 50);
    }

    #[test]
    fn software_age_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_age(100);
        assert_eq!(software._age_of_code, 100);
    }

    #[test]
    fn software_complexity_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_complexity(100);
        assert_eq!(software._complexity_of_code, 100);

        software.remove_complexity(50);
        assert_eq!(software._complexity_of_code, 50);
    }


}