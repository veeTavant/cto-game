use rand::Rng;
use crate::world::timeframe::YearWeek;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Architecture {
    ProofofConcept /* ,
    Monolith,
    Microservices,
    EventDriven */
}

pub enum MonetizationModel {
 /*   OpenSource,
    Freemium,
    FreeTier, */
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
    _customers: u16,                // number of customers
    _active_users: u32,             // number of active users
    _capacity_percentage_active_users: u16,             // number of active users (not same as customers)
    _percentage_free_users: u16,    // percentage of free users
    _monetization_model:    MonetizationModel,
    _releases:              u16,    // how many releases have their been
    _last_release_yearweek: YearWeek,
    _reliability:           u16,    // 0 - 100
    _technical_debt:        u16,    // 0 - 100
    _quality:               u16
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
                          _active_users: 0,
                          _capacity_percentage_active_users: 0,            // 0 - 100 percentage to capacity - but can be over capacity too
                          _percentage_free_users: 0,
                          _monetization_model: MonetizationModel::Proprietary,
                          _releases: 0,
                          _last_release_yearweek: YearWeek::new(2000, 1),
                          _reliability: 100,                               // 0 - 100 - as reported by users - not testers
                          _technical_debt: 0,                              // 0 - 100 - dependent upon development and new feature pace
                          _quality: 100                                    // 0 - 100 - as measured by testers
                        };
    }


    pub fn quality(&self) -> u16 {
        self._quality
    }

    // Measure of quality from complexity_of_code, feature_richness, ease_of_use, components, services, dependencies, architecture
    //
    pub fn recalculate_quality(&mut self, number_of_devs: u16, number_of_testers: u16) {
        if self._complexity_of_code < 10 && self._services < 2 && self._components < 5 {
            self._quality = 100;
        } else {

            let manageable_codebase_limit = 80000;

            if self._lines_of_code > manageable_codebase_limit {
                //let recalculation_factor = 0.0f32;
                
                if number_of_devs > number_of_testers {
                    let modifier = ( ( (number_of_devs - number_of_testers) as f32 ) / 3.0f32 ) as u16;

                    if modifier < self._quality {
                        self._quality -= modifier;
                    } else {
                        self._quality = 0;
                    } 

                } else { // more testers - improve quality
                    let modifier = ( ( (number_of_testers - number_of_devs) as f32 ) / 3.0f32 ) as u16;

                    if self._quality + modifier < 100 {
                        self._quality += modifier;
                    } else {
                        self._quality = 100;
                    }
                }
            }
        }
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
    //
    pub fn market_popularity(&self, current_yearweek: &YearWeek) -> u16 {

        if self._releases == 0 {
            return 0
        }

        let mut popularity = ( self._customers as i16 + self._capacity_percentage_active_users as i16 ) / 2;
        
        // Now when was the last release?
        //
        let difference_weeks = self._last_release_yearweek.difference_weeks(current_yearweek);

        if difference_weeks < 1 {
            return popularity as u16
        } else if difference_weeks < 10  {
            popularity += 15
        } else if difference_weeks < 20 {
            popularity += 5
        } else if difference_weeks < 40 {
            popularity /= 2
        } else {
            popularity /= 4
        }

        return popularity as u16

    }

    pub fn lines_of_code(&self) -> u32 {
        self._lines_of_code
    }

    pub fn releases(&self) -> u16 {
        self._releases
    }

    pub fn reliability(&self) -> u16 {
        self._reliability
    }

    pub fn add_customers(&mut self, customers: u16, b2b: bool) {
        self._customers += customers;

        // Adjust users according to b2b selling
        //
        if b2b {
            let mut rng = rand::thread_rng();
            let rand_factor: f32 = rng.gen();
            let rand_users: f32 = rng.gen();
    
            self._active_users += ( customers as f32 * rand_factor * rand_users * 20.0f32 ) as u32

        } else {
            self._active_users += customers as u32;
        }
    }

    pub fn remove_customers(&mut self, customers: u16) {

        if customers > self._customers {
            // remove a percentage of active users
            //
            let factor = customers as f64 / self._customers as f64;
            self._active_users /= ( self._active_users as f64 * factor) as u32;
            self._customers -= customers
        } else {
            self._customers = 0;
            self._active_users = 0;
        }
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


    // Three key methods
    //
    // - Work on Features
    // - Refactor
    // - Bug Fix
    //

    // Work on features
    //
    // Finger very much in the air
    pub fn work_on_features(&mut self, number_of_devs: u16, dev_focus: u16, days: u16) {
        let loc_per_day = 250;
        self._lines_of_code += ( number_of_devs as f32 * (dev_focus as f32 / 100.0f32 ) ) as u32 * loc_per_day * days as u32;

        self.recalculate_code_complexity(number_of_devs, dev_focus);
    }
/* 
    pub fn refactor(&mut self) {

    }

    pub fn bug_fix(&mut self) {

    }
*/
    // Try and keep dev_focus 0 - 100
    //
    fn recalculate_code_complexity(&mut self, number_of_devs: u16, _dev_focus: u16) {

        // Size of codebase per developer
        //
        let lines_per_dev = ( self._lines_of_code as f32 / number_of_devs as f32 ) as u32;

        // Lower dev focus means more complexity but even 100% dev focus can't fix a huge codebase
        // with not enough people.  What's the ratio?
        //
        let arbitrary_code_limit = 150000;

        self._complexity_of_code = ( 30.00f32 * lines_per_dev as f32 / arbitrary_code_limit as f32 ) as u16;

        // We limit to a third size
        //
     }


    pub fn customers(&self) -> u16 {
        self._customers
    }


}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn software_customers_tests() {
        let mut software = Software::new(0, 0, 0, 0);

        software.add_customers(20, false);
        assert_eq!(software.customers(), 20);
    }


}