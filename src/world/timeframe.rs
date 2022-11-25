use chrono::{DateTime};
use chrono::{Local};

pub struct Timeframe {
    _speed: u16,                          // 1-1000 - millisecond loop time (lower is faster)
    _game_ticks: u32,                     // how far we're into the game
    _last_tick_time: DateTime<Local>,     // where we are now
    _game_start_time: DateTime<Local>,    // when did the game start?

    _frames_per_week: u16,                // frames of game time in a week
    _weeks_per_year: u16,                 // number of weeks in a year

    //_start_date: String                   // YYYYMMDD

}

impl Timeframe {
    pub fn new(speed: u16, game_ticks: u32) -> Timeframe {
        Timeframe { _speed: speed, _game_ticks: game_ticks, _last_tick_time: Local::now(), _game_start_time: Local::now(), _frames_per_week: 2, _weeks_per_year: 52, /* _start_date: */  }
    }

    pub fn frames_per_week(&self) -> u16 {
        self._frames_per_week
    }
} 


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn timeframe_tests() {

        let timeframe = Timeframe::new(100, 100);
        assert_eq!(timeframe.frames_per_week() , 52);
 
    }
 
} 