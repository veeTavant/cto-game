use chrono::{DateTime, Date};
use chrono::{Local};

// World for our Software and Company to live in
//
pub struct World {
    pub _global_economic_factors: u16,    // 0-1000
    pub _competition_in_market: u16,      // 0-1000
    pub _job_market: u16,                 // 0-1000
    pub _speed: u16,                      // 1-1000 - millisecond loop time (lower is faster)
    pub _game_ticks: u32,                 // how far we're into the game
    pub _last_tick_time: DateTime<Local>, // where we are now

    pub _game_start_time: DateTime<Local>     // when did the game start?

    // Game mechanics
    //
    // Pass the other objects to ensure model is updated
    //

    // Counters for various things
    //
}

impl World {
    fn new (a: u16, b: u16, c: u16, d: u16, e: u32, f: DateTime<Local>) -> Self {
        World {
            _global_economic_factors: a,
            _competition_in_market: b,
            _job_market: c,
            _speed: d,
            _game_ticks: e,
            _last_tick_time: f,
            _game_start_time: Local::now()
        }
    }
    pub fn increment_game_ticks(&mut self) {
        self._game_ticks = self._game_ticks + 1;
    }

    pub fn set_current_time(&mut self, time_now: DateTime<Local>) {
        self._last_tick_time = time_now;
    }

    pub fn get_game_elapse_time(& self) -> chrono::Duration {
        return (self._game_start_time - self._last_tick_time);
    }


    fn do_game_update (&mut self) {

    }

    
}

