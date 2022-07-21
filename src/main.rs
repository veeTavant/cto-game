// https://github.com/ihalila/pancurses
//
extern crate pancurses;
use pancurses::*;

use std::collections::HashMap;

pub mod employee;
use crate::employee::Employee;
use crate::employee::EmployeeType;

use chrono::{Utc, Local};

fn main() {
  let window = initscr();

  let mut employees = HashMap::new();

  let user1 = Employee {
    _type: EmployeeType::Developer,
    _name: "Richard".to_string(),
    _age: 50,
    _efficiency: 100,
    _salary: 250,
    _talent: 100
  };

  employees.insert(
    user1._name.to_string(),
    user1
  );


  let mut r = window.get_max_y() - 4;
  let mut c = window.get_max_x() - 4;

  let mut outputYString: String = "Y max".to_owned();
  let mut yMaxString: String = r.to_string().to_owned();
  outputYString.push_str(&yMaxString);


  window.mvaddstr(1, 0, "Employees:");
  window.mvaddstr(2, 0, "Developers:");

  curs_set(0);

  window.refresh();

  // set non-blocking mode
  //
  window.timeout(100);
  window.keypad(true);
  noecho();

  let mut quitting : bool = false;
  let mut commandString : String = String::new();

  loop {
      match window.getch() {
          Some(Input::Character(c)) => {
              
            // Check for escape
            if c == '\u{1b}' {
              quitting = true;
              commandString = String::from("QUITTING");
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

      let formattedTime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      // Update time and zap to CMD prompt
      //
      window.mvaddstr(0, window.get_max_x() - 20, formattedTime);

      if quitting {
        window.mvaddstr(window.get_max_y() - 1 , 0, "CLOSING");
      } else {
        
        let mut owned_string: String = "CMD> ".to_owned();
        owned_string.push_str(&commandString);

        window.mvaddstr( window.get_max_y() - 1 , 0, owned_string);
      }
      
      window.refresh();

  }

  endwin();
}