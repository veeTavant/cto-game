use chrono::{DateTime};
use chrono::{Local};

// World for our Software and Company to live in
//
pub struct World {
    _global_economic_factors: u16,        // 0-1000
    _competition_in_market: u16,          // 0-1000
    _job_market: u16,                     // 0-1000
    _speed: u16,                          // 1-1000 - millisecond loop time (lower is faster)
    _game_ticks: u32,                     // how far we're into the game
    _last_tick_time: DateTime<Local>,     // where we are now
    _game_start_time: DateTime<Local>,    // when did the game start?


    _frames_per_week: u16,                // frames of game time in a week
    _weeks_per_year: u16                  // number of weeks in a year

    // Game mechanics
    //
    // Pass the other objects to ensure model is updated
    //

    // Counters for various things
    //
}

impl World {

    pub fn new(global_economic_factors :u16, competition_in_market :u16, job_market :u16, speed :u16, game_ticks :u32) -> World {   
        return World { _global_economic_factors: global_economic_factors, _competition_in_market: competition_in_market, _job_market: job_market, _speed: speed, _game_ticks: game_ticks, _last_tick_time: Local::now(), _game_start_time: Local::now(), _frames_per_week: 2, _weeks_per_year: 52 };
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
        self._speed
    }

    pub fn game_ticks(&self) -> u32 {
        self._game_ticks
    }

    pub fn last_tick_time(&self) -> DateTime<Local> {
        self._last_tick_time
    }

    //pub fn game_start_time(&self) -> DateTime<Local> {
    //    self._game_start_time
    //}
    
    pub fn increment_game_ticks(&mut self) {
        self._game_ticks = self._game_ticks + 1;

        // run the update
        self.do_game_update();
    }

    pub fn set_current_time(&mut self, time_now: DateTime<Local>) {
        self._last_tick_time = time_now;

    }

    pub fn get_game_elapse_time(& self) -> chrono::Duration {
        return self._game_start_time - self._last_tick_time;
    }


    fn do_game_update (&mut self) {

    }

    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_tests() {

        let mut world = World::new(100, 100, 100, 100, 0);

        world.increment_game_ticks();
        assert_eq!(world._game_ticks, 1);
    }


}
