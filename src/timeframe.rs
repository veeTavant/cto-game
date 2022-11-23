use chrono::{DateTime};
use chrono::{Local};

struct Timeframe {
    _speed: u16,                          // 1-1000 - millisecond loop time (lower is faster)
    _game_ticks: u32,                     // how far we're into the game
    _last_tick_time: DateTime<Local>,     // where we are now
    _game_start_time: DateTime<Local>,    // when did the game start?

    _frames_per_week: u16,                // frames of game time in a week
    _weeks_per_year: u16,                 // number of weeks in a year

    _start_date: String                   // YYYYMMDD

}

impl Timeframe {
    pub fn new(speed: u16, game_ticks: u32) -> Timeframe {
        Timeframe{speed, game_ticks, DateTime<Local>::now, DateTime<Local>::now, 2, 52, String("20221123") }
    }
}


#[cfg(test)]
mod test {

    

}