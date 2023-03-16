
use chrono::{DateTime, Local, NaiveDate, Weekday, Datelike};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct YearWeek {
    _year: i32,
    _week: u32
}


impl YearWeek {
    pub fn new(year: i32, week: u32) -> YearWeek {
        assert!(week >=1 && week <= 52);
        YearWeek { _year: year, _week: week }
    }

    pub fn difference_weeks(&self, year_week: &YearWeek) -> u32 {
        i32::abs_diff(self._year * 52 + self._week as i32, year_week._year * 52 + year_week._week as i32) 
    }

    // Returns true if the month rotates to cause a payroll run
    //
    pub fn increment_week(&mut self) -> bool {

        //println!("creating before_date = {}/{}",self._year, self._week);

        let before_date = NaiveDate::from_isoywd(self._year, self._week, Weekday::Mon);
        //println!("before_date = {}",before_date);

        if self._week < 52 {
            self._week += 1;
        } else  {
            self._week = 1;
            self._year += 1;
        }

        let after_date = NaiveDate::from_isoywd(self._year, self._week, Weekday::Mon);
        //println!("after_date = {}",after_date);

        return before_date.month() != after_date.month()
    }

    pub fn to_string(&self) -> String {
        format!("{:04}-{:02}", self._year, self._week)
    }

    
}

pub struct Timeframe {
    _speed: u16,                          // 1-1000 - millisecond loop time (lower is faster)
    _game_ticks: u32,                     // how far we're into the game
    _last_tick_time: DateTime<Local>,     // where we are now
    _game_start_time: DateTime<Local>,    // when did the game start?
    _ticks_per_week: u16,                 // ticks of game time in a week
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
                    _ticks_per_week: 2,
                    _weeks_per_year: 52,
                    _start_yearweek: YearWeek::new(2000, 1),
                    _current_yearweek: YearWeek::new(2000, 1)
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
    
    pub fn ticks_per_week(&self) -> u16 {
        self._ticks_per_week
    }
    
    pub fn increment_game_ticks(&mut self) -> bool {
        self._game_ticks += 1;

        if self._game_ticks % self._ticks_per_week as u32 == 0 {
            return self._current_yearweek.increment_week()
        }

        return false
    }

    pub fn set_current_time(&mut self, time_now: DateTime<Local>) {
        self._last_tick_time = time_now
    }

    pub fn get_game_elapse_time(&self) -> chrono::Duration {
        return self._game_start_time - self._last_tick_time
    }

    pub fn get_current_yearweek(&self) -> YearWeek {
        self._current_yearweek
    }

    pub fn get_current_month(&self)  -> u32 {
        return NaiveDate::from_isoywd(self._current_yearweek._year, self._current_yearweek._week, Weekday::Mon).month()
    }

    pub fn get_current_year(&self) -> i32 {
        return self._current_yearweek._year
    }

    pub fn get_current_week(&self) -> u32 {
        return self._current_yearweek._week
    }

} 


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn timeframe_tests() {

        let timeframe = Timeframe::new(100, 100);
        assert_eq!(timeframe.ticks_per_week() , 2);
 
    }

    #[test]
    fn year_week_test() {
        let year_week = YearWeek::new(2000, 1);
        assert_eq!(year_week.to_string(), "2000-01");
    }

    #[test]
    fn year_week_difference_test() {
        let year_week_1 = YearWeek::new(2000, 1);
        let year_week_2 = YearWeek::new(2010, 40);
        println!("Difference = {}",year_week_1.difference_weeks(&year_week_2));
        assert_eq!(year_week_1.difference_weeks(&year_week_2) , 559);
    }

    #[test]
    fn test_month() {
        let test_month = Timeframe::new(100, 100);
        assert_eq!(test_month.get_current_month(), 1);
    }

    #[test]
    fn test_week() {
        let test_week: Timeframe = Timeframe::new(100, 100);
        assert_eq!(test_week.get_current_week(), 1);
    }

 
} 
