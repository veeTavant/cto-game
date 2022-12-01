
use chrono::{DateTime};
use chrono::{Local};


pub struct YearWeek {
    _year: u32,
    _week: u32
}

impl YearWeek {
    pub fn new(year: u32, week: u32) -> YearWeek {
        YearWeek { _year: year, _week: week }
    }

    pub fn difference_weeks(&self, year_week: &YearWeek) -> u32 {
       
        //println!("Year = {}, Week = {}", self._year, self._week);
        //println!("Year = {}, Week = {}", year_week._year, year_week._week);

        u32::abs_diff((self._year * 52 + self._week), (year_week._year * 52 + year_week._week)) 
//        ((self._year - year_week._year) * 52) + (self._week - year_week._week) 
    }

    pub fn year(&self) -> u32 {
        self._year
    }

    pub fn week(&self) -> u32 {
        self._week
    }
}

pub struct Timeframe {
    _speed: u16,                          // 1-1000 - millisecond loop time (lower is faster)
    _game_ticks: u32,                     // how far we're into the game
    _last_tick_time: DateTime<Local>,     // where we are now
    _game_start_time: DateTime<Local>,    // when did the game start?
    _frames_per_week: u16,                // frames of game time in a week
    _weeks_per_year: u16,                 // number of weeks in a year
    _start_yearweek: YearWeek,            // week we started
    _current_yearweek: YearWeek,          // current year
}

impl Timeframe {
    pub fn new(speed: u16, game_ticks: u32) -> Timeframe {
        Timeframe { _speed: speed,
                    _game_ticks: game_ticks,
                    _last_tick_time: Local::now(),
                    _game_start_time: Local::now(),
                    _frames_per_week: 2,
                    _weeks_per_year: 52,
                    _start_yearweek: YearWeek::new(2000, 0),
                    _current_yearweek: YearWeek::new(2000, 0)
                  }
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

    pub fn game_start_time(&self) -> DateTime<Local> {
        self._game_start_time
    }

    pub fn frames_per_week(&self) -> u16 {
        self._frames_per_week
    }

    pub fn weeks_per_year(&self) -> u16 {
        self._weeks_per_year
    }
    
    pub fn increment_game_ticks(&mut self) {
        self._game_ticks += 1
    }

    pub fn set_current_time(&mut self, time_now: DateTime<Local>) {
        self._last_tick_time = time_now
    }

    pub fn get_game_elapse_time(&self) -> chrono::Duration {
        return self._game_start_time - self._last_tick_time
    }
} 


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn timeframe_tests() {

        let timeframe = Timeframe::new(100, 100);
        assert_eq!(timeframe.frames_per_week() , 2);
 
    }

    #[test]
    fn year_week_test() {
        let year_week = YearWeek::new(2000, 1);
        assert_eq!(year_week.year() , 2000);

    }

    #[test]
    fn year_week_difference_test() {
        let year_week_1 = YearWeek::new(2000, 1);
        let year_week_2 = YearWeek::new(2010, 40);
        println!("Difference = {}",year_week_1.difference_weeks(&year_week_2));
        assert_eq!(year_week_1.difference_weeks(&year_week_2) , 559);

    }
 
} 
