// https://github.com/ihalila/pancurses
//
extern crate pancurses;
//use chrono::format::Numeric;
use pancurses::*;

use std::collections::HashMap;

pub mod employee;
use crate::employee::Employee;
use crate::employee::EmployeeType;

use chrono::{Local};



/*
fn boxed_employee() -> Box<Employee> {

  return Box::new(Employee { _type: EmployeeType::Administrator, _name: "CTO".to_string(), 
  _age: 48,
  _salary: 100,
  _efficiency: 250,
  _talent: 100 });

}
*/

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

//  let boxed_employee = boxed_employee();
  _employees.insert("Developer 1", dev1);
  _employees.insert("Developer 2", dev2);
  _employees.insert("Developer 3", dev3);

}

fn main() {

  let mut employees = HashMap::new();
  build_company(&mut employees);

  let window = initscr();

  let number_of_employees = employees.keys().len().to_string();
  window.mvaddstr(1, 20, number_of_employees);

  // Count developers
  //for employee in &employees {
   // if &employee.ref._
  //}


  window.mvaddstr(1, 0, "Employees:");
  window.mvaddstr(2, 0, "Developers:");

  curs_set(0);

  window.refresh();

  // set non-blocking mode
  //
  window.timeout(100);
  window.keypad(true);
  noecho();

  let command_string : String = String::new();

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

      let format_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      // Update time and zap to CMD prompt
      //
      window.mvaddstr(0, window.get_max_x() - 20, format_time);

      let mut owned_string: String = "CMD> ".to_owned();
      owned_string.push_str(&command_string);

      window.mvaddstr( window.get_max_y() - 1 , 0, owned_string);
      
      window.refresh();

  }

  endwin();
}