// https://github.com/ihalila/pancurses
//
extern crate pancurses;
//use chrono::format::Numeric;
use pancurses::*;

use std::char::ToUppercase;
use std::collections::HashMap;
use std::convert::TryInto;

pub mod employee;
pub mod software;
pub mod world;
pub mod company;

use crate::company::Company;
use crate::software::Software;
use crate::world::World;
use crate::employee::Employee;
use crate::employee::EmployeeType;

use chrono::{Local};

fn build_company(_employees: &mut HashMap<&str, Box<Employee>>) {
  
  let dev1 = Box::new(Employee {
    _type: EmployeeType::Developer,
    _name: "Developer 1".to_string(),
    _age: 50,
    _efficiency: 90,
    _salary: 200,
    _talent: 90
  });

  let dev2 = Box::new(Employee {
    _type: EmployeeType::Developer,
    _name: "Developer 2".to_string(),
    _age: 23,
    _efficiency: 35,
    _salary: 89,
    _talent: 77
  });

  let dev3 = Box::new(Employee {
    _type: EmployeeType::Developer,
    _name: "Developer 3".to_string(),
    _age: 30,
    _efficiency: 70,
    _salary: 100,
    _talent: 85
  });

  _employees.insert("Developer 1", dev1);
  _employees.insert("Developer 2", dev2);
  _employees.insert("Developer 3", dev3);
  _employees.insert("Admin 1", Box::new(Employee { _type: EmployeeType::Administrator, _name: "Admin 1".to_string(), _age: 37, _efficiency: 80, _salary: 80, _talent: 65}));

}

fn draw_hud(_employees: &HashMap<&str, Box<Employee>>, _company: &Company, _software: &Software, _world: &World, _window: &Window) {

  _window.mvaddstr(1, 1, "Employees:");
  _window.mvaddstr(2, 1, "Developers:");
  _window.mvaddstr(3, 1, "Testers:");
  _window.mvaddstr(4, 1, "Administrators:");
  _window.mvaddstr(5, 1, "Marketers:");
  _window.mvaddstr(6, 1, "Salespeople:");

  let mut developers = 0;
  let mut testers = 0;
  let mut administrators = 0;
  let mut marketers = 0;
  let mut salespeople = 0;


  // Count developers
  for (k, v) in _employees.iter() {

    if (v._type == EmployeeType::Developer) {
      developers = developers + 1;
    } else if (v._type == EmployeeType::Administrator) {
      administrators = administrators + 1;
    } else if (v._type == EmployeeType::Tester) {
      testers = testers + 1;
    } else if (v._type == EmployeeType::Salesperson) {
      salespeople = salespeople + 1;
    } else if (v._type == EmployeeType::Marketeer) {
      marketers = marketers + 1;
    }
  }

  let first_column_results_pos = 30;
  let number_of_employees = _employees.keys().len().to_string();
  _window.mvaddstr(1, first_column_results_pos, number_of_employees);
  _window.mvaddstr(2, first_column_results_pos, developers.to_string());
  _window.mvaddstr(3, first_column_results_pos, testers.to_string());
  _window.mvaddstr(4, first_column_results_pos, administrators.to_string());
  _window.mvaddstr(5, first_column_results_pos, marketers.to_string());
  _window.mvaddstr(6, first_column_results_pos, salespeople.to_string());

  let second_column_pos  = _window.get_max_x() / 2;
  _window.mvaddstr(1, second_column_pos  , "Cash In Bank:");
  _window.mvaddstr(2, second_column_pos  , "Customers:");
  _window.mvaddstr(3, second_column_pos  , "Retail Price:");
  _window.mvaddstr(4, second_column_pos  , "Lines of Code:");
  _window.mvaddstr(5, second_column_pos  , "Age of Code:");
  _window.mvaddstr(6, second_column_pos  , "Code Complexity:");

  let second_column_results_pos = second_column_pos + 30;
  _window.mvaddstr(1, second_column_results_pos, _company._cash_in_bank.to_string());
  _window.mvaddstr(2, second_column_results_pos, _company._customers.to_string());
  _window.mvaddstr(3, second_column_results_pos, _company._cost_of_service_per_month.to_string());
  _window.mvaddstr(4, second_column_results_pos, _software._lines_of_code.to_string());
  _window.mvaddstr(5, second_column_results_pos, _software._age_of_code.to_string());
  _window.mvaddstr(6, second_column_results_pos, _software._complexity_of_code.to_string());


  // World
  //

  _window.mvaddstr(_window.get_max_y() - 6, 1, "Global Economony:");
  _window.mvaddstr(_window.get_max_y() - 5, 1, "Competition:");
  _window.mvaddstr(_window.get_max_y() - 4, 1, "Job Market:");
  _window.mvaddstr(_window.get_max_y() - 3, 1, "Speed Factor:");

  _window.mvaddstr(_window.get_max_y() - 6, second_column_pos, "Game Frame:");
  _window.mvaddstr(_window.get_max_y() - 7, second_column_pos, "Game Time:");
  


  _window.mvaddstr(_window.get_max_y() - 6, first_column_results_pos, _world._global_economic_factors.to_string());
  _window.mvaddstr(_window.get_max_y() - 5, first_column_results_pos, _world._competition_in_market.to_string());
  _window.mvaddstr(_window.get_max_y() - 4, first_column_results_pos, _world._job_market.to_string());
  _window.mvaddstr(_window.get_max_y() - 3, first_column_results_pos, _world._speed.to_string());

  _window.mvaddstr(_window.get_max_y() - 6, second_column_results_pos, _world._game_ticks.to_string());
  _window.mvaddstr(_window.get_max_y() - 7, second_column_results_pos, _world.get_game_elapse_time().to_string());

}

fn draw_matrix_workface(_employees: &HashMap<&str, Box<Employee>>, _company: &Company, _software: &Software, _world: &World, _window: &Window) {

  let min_x  = _window.get_max_x() / 2 - _window.get_max_x() / 4;
  let max_x  = _window.get_max_x() / 2 + _window.get_max_x() / 4;
  let min_y = 8;
  let max_y = min_y + 20;

  let horiz_string = std::iter::repeat("-").take((1 + _window.get_max_x() / 2) as usize).collect::<String>();
  let horiz_string_2 = horiz_string.clone();
  _window.mvaddstr(min_y, min_x, horiz_string );
  _window.mvaddstr(max_y, min_x, horiz_string_2 );

  // Box it out
  //
  for y_pos in min_y + 1 ..max_y {
    _window.mvaddch(y_pos, min_x, '|');
    _window.mvaddch(y_pos, max_x, '|');
  } 

}

fn main() {

  let mut employees = HashMap::new();
  build_company(&mut employees);

  let software: Box<Software> = Box::new(Software {
    _lines_of_code: 0,
    _age_of_code: 0,
    _complexity_of_code: 0
  });

  let mut world: Box<World> = Box::new(World {
    _competition_in_market: 100,
    _global_economic_factors: 100,
    _job_market: 100,
    _speed: 100,
    _game_ticks: 0,
    _last_tick_time: Local::now(),
    _game_start_time: Local::now()
  });

//  let mut world2 = World(100, 100, 100, 100, 0);
 
  let company: Box<Company> = Box::new(Company {
    _cash_in_bank: 1000000,
    _cost_of_service_per_month: 30,
    _customers: 0
  });

  // Init windows
  //
  let window = initscr();

  draw_hud(&employees, &company, &software, &world, &window);
  draw_matrix_workface(&employees, &company, &software, &world, &window);

  curs_set(0);
  window.refresh();

  // set non-blocking mode
  //
  window.timeout(world._speed as i32);
  window.keypad(true);
  noecho();

  let command_string : String = String::new();

  // Store game time
  //
  loop {
      match window.getch() {
          Some(Input::Character(c)) => {
              
            // Check for escape
            if c == '\u{1b}' {
              //quitting = true;
              //command_string = String::from("QUITTING");
              break;
            }
            window.addch(c);

          }
          Some(Input::KeyDC) => break,
          Some(input) => {
              window.addstr(&format!("{:?}", input));
          }
          None => (),
      }

      // Format time and update
      //
      let format_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      window.mvaddstr(0, window.get_max_x() - 20, format_time);

      if (Local::now() > world._last_tick_time) {
        world.increment_game_ticks();
        world.set_current_time(Local::now());
        draw_hud(&employees, &company, &software, &world, &window);
      }

      // CMD prompt
      //
      let mut owned_string: String = "CMD> ".to_owned();
      owned_string.push_str(&command_string);

      window.mvaddstr( window.get_max_y() - 1 , 0, owned_string);
      window.refresh();
  }

  endwin();
}