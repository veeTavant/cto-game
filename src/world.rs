use chrono::{DateTime};
use chrono::{Local};

pub mod Timeframe;

use super::Company;
use super::Software;


// World for our Software and Company to live in
//
pub struct World {
    _global_economic_factors: u16,        // 0-1000
    _competition_in_market: u16,          // 0-1000
    _job_market: u16,                     // 0-1000
    _timeframe: Timeframe::Timeframe
}

impl World {

    pub fn new(global_economic_factors :u16, competition_in_market :u16, job_market :u16, speed :u16, game_ticks :u32) -> World {   
        return World { _global_economic_factors: global_economic_factors, _competition_in_market: competition_in_market, _job_market: job_market, _timeframe: Timeframe::Timeframe::new(speed, game_ticks) };
    }

    pub fn global_economic_factors(& self) -> u16 {
        self._global_economic_factors
    }

    pub fn competition_in_market(&self) -> u16 {
        self._competition_in_market
    }

    pub fn job_market(&self) -> u16 {
        self._job_market
    }

    pub fn speed(&self) -> u16 {
        self._timeframe.speed()
    }

    pub fn game_ticks(&self) -> u32 {
        self._timeframe.game_ticks()
    }

    pub fn last_tick_time(&self) -> DateTime<Local> {
        self._timeframe.last_tick_time()
    }

    //pub fn game_start_time(&self) -> DateTime<Local> {
    //    self._game_start_time
    //}
    
    pub fn increment_game_ticks(&mut self, company: &Company, software: &Software, time_now: DateTime<Local>) {
        
        // 
        self._timeframe.increment_game_ticks();

        // run the update
        self.do_game_update(company, software);

        // Update time
        //
        self._timeframe.set_current_time(time_now);
    }

    pub fn get_game_elapse_time(& self) -> chrono::Duration {
        self._timeframe.get_game_elapse_time()
    }


    // Process a tick of progress (how do we work this out?)
    //
    // The world defines how our software and company do the update and what that
    // means to the company - so the logical placing of everything appears right at
    // the moment.
    //
    fn do_game_update (&mut self, company: &Company, software: &Software) {
    
        // Find out where our software is like, what our company mission is currently and how the world is reacting to it
        //
        // Basic Steps for one game cycle
        //
        // 1. Usage / Customer Move (usage changes, usage stats, active user counts, feedback, problems)
        // 2. Operations Move (what are customer interactions like with current system)
        // 3. Software Move (what do you add, take, change - development move) 
        // 4. (Company level) Management Move (Direction, MarketingStrategy, Ownership, ?)
        // 5. HR Move (HiringStrategy accoriding to management - any hiring/firing/loss updates)
        // 6. Sales Update (new users, lost users)
        // 7. Out Of Band Occurrences (Audit, Certification, Renewals, Power Outages, Hack Attacks)
        // 8. Accounting updates (payments, receipts and cashflow update)
        // 9. Ready for Release?
        
//        if software.releases() > 

        if software.usability_factor() > 0 {
            // What's the age of the software
            //

        }


        // What is the factor of usefulness for software? software.usability_factor()
        // 
        // Features and Ease of Use
        //

/*

        // How many customers do we have? How many active users?
        //
        match company.direction()
        {
            CompanyDirection::B2B => {

                // Is the software usable by business?
                //
                if software.usability_factor() > 30 {



                }
            },
            CompanyDirection::B2C => {

                // Is the software usable by individuals?
                //
                if software.usability_factor() > 10 {

                }
            }
        } 
        */
        
/* 
        match software.get_architecture() {
            Architecture::ProofofConcept => {

            },
            Architecture::EventDriven => {

            },
            Architecture::Microservices => {

            },
            Architecture::Monolith => {

            }
        }
*/

//        software.complexity_of_code()
    }

    
}

#[cfg(test)]
mod test {
    use crate::company::CompanyDirection;
    use super::*;

    #[test]
    fn time_tests() {

        let mut world = World::new(100, 100, 100, 100, 0);

        let company: Company = Company::new(100, CompanyDirection::B2B);
        let software: Software = Software::new(100, 100, 100, 100);
        world.increment_game_ticks(&company, &software, Local::now());
        assert_eq!(world.game_ticks(), 1);
    }


}
