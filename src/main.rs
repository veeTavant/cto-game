// https://github.com/ihalila/pancurses
//
extern crate pancurses;
use pancurses::*;

use crate::company::Company;
use crate::software::Software;
use crate::world::World;
use crate::employee::Employee;
use crate::employee::EmployeeType;

use chrono::Local;

mod employee;
mod software;
mod world;
mod company;


fn draw_hud(_company: &Company, _software: &Software, _world: &World, _window: &Window) {

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
  for (_k, v) in _company.get_employees().iter() {

    if v.employee_type() == EmployeeType::Developer {
      developers = developers + 1;
    } else if v.employee_type() == EmployeeType::Administrator {
      administrators = administrators + 1;
    } else if v.employee_type() == EmployeeType::Tester {
      testers = testers + 1;
    } else if v.employee_type() == EmployeeType::Salesperson {
      salespeople = salespeople + 1;
    } else if v.employee_type() == EmployeeType::Marketeer {
      marketers = marketers + 1;
    }
  }

  let first_column_results_pos = 30;

  let number_of_employees = _company.get_employees().keys().len().to_string();
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
  _window.mvaddstr(1, second_column_results_pos, _company.cash_in_bank().to_string());
  _window.mvaddstr(2, second_column_results_pos, _software.customers().to_string());
  _window.mvaddstr(3, second_column_results_pos, _software.cost_of_service().to_string());
  _window.mvaddstr(4, second_column_results_pos, _software.lines_of_code().to_string());
  _window.mvaddstr(5, second_column_results_pos, _software.age_of_code().to_string());
  _window.mvaddstr(6, second_column_results_pos, _software.complexity_of_code().to_string());


  // World
  //

  _window.mvaddstr(_window.get_max_y() - 6, 1, "Global Economony:");
  _window.mvaddstr(_window.get_max_y() - 5, 1, "Competition:");
  _window.mvaddstr(_window.get_max_y() - 4, 1, "Job Market:");
  _window.mvaddstr(_window.get_max_y() - 3, 1, "Speed Factor:");

  _window.mvaddstr(_window.get_max_y() - 6, second_column_pos, "Game Frame:");
  _window.mvaddstr(_window.get_max_y() - 7, second_column_pos, "Game Time:");
  


  _window.mvaddstr(_window.get_max_y() - 6, first_column_results_pos, _world.global_economic_factors().to_string());
  _window.mvaddstr(_window.get_max_y() - 5, first_column_results_pos, _world.competition_in_market().to_string());
  _window.mvaddstr(_window.get_max_y() - 4, first_column_results_pos, _world.job_market().to_string());
  _window.mvaddstr(_window.get_max_y() - 3, first_column_results_pos, _world.speed().to_string());

  _window.mvaddstr(_window.get_max_y() - 6, second_column_results_pos, _world.game_ticks().to_string());
  _window.mvaddstr(_window.get_max_y() - 7, second_column_results_pos, _world.get_game_elapse_time().to_string());

}

fn draw_matrix_workface(mut _company: &Company, _software: &Software, _world: &World, _window: &Window) {

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

  let software = Software::new(0, 0, 0, 0);
  let mut world = World::new(100, 100, 100, 100, 0);
  let mut company = company::Company::new(100, company::CompanyDirection::B2B);

  let dev1 = Employee::new(EmployeeType::Developer, 1, "Developer 1".to_string(), 50, 90, 200, 90);
  let dev2 = Employee::new(EmployeeType::Developer, 2,  "Developer 2".to_string(), 23, 35, 89, 77);
  let dev3 = Employee::new(EmployeeType::Developer, 3, "Developer 3".to_string(), 30, 70, 100, 85);
  let admin1 = Employee::new(EmployeeType::Administrator, 4, "Admin 1".to_string(), 37,  80,  80,  65);

  company.add_employee(dev1);
  company.add_employee(dev2);
  company.add_employee(dev3);
  company.add_employee(admin1);

  // Init windows
  //
  let window = initscr();

  draw_hud(&company, &software, &world, &window);
  draw_matrix_workface(&company, &software, &world, &window);

  curs_set(0);
  window.refresh();

  // set non-blocking mode
  //
  window.timeout(world.speed() as i32);
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

      if Local::now() > world.last_tick_time() {
        world.increment_game_ticks(&company, &software, Local::now());

        draw_hud(&company, &software, &world, &window);
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn world_tests() {

      let world = World::new(100, 100, 100, 100, 0);
      assert_eq!(world.game_ticks(), 0);

    }

}
