// https://github.com/ihalila/pancurses
//
extern crate pancurses;
use pancurses::*;

use std::collections::HashMap;

pub mod employee;
use crate::employee::Employee;
use crate::employee::EmployeeType;

use chrono::{Local};

fn build_company(_employees: &mut HashMap<&str, &Employee>) {
  
  let user1 = Employee {
    _type: EmployeeType::Developer,
    _name: "Richard".to_string(),
    _age: 50,
    _efficiency: 100,
    _salary: 250,
    _talent: 100
  };

//  _employees.insert("string", &user1);

}

fn main() {

  let mut employees = HashMap::new();
  build_company(&mut employees);

  let user1 = Employee {
    _type: EmployeeType::Developer,
    _name: "Richard".to_string(),
    _age: 50,
    _efficiency: 100,
    _salary: 250,
    _talent: 100
  };

  employees.insert("string", &user1);

  let window = initscr();

  //let r = window.get_max_y() - 4;
  //let c = window.get_max_x() - 4;
  //let mut output_y_string: String = "Y max".to_owned();
  //let mut y_max_string: String = r.to_string().to_owned();
  //output_y_string.push_str(&y_max_string);


  window.mvaddstr(1, 0, "Employees:");
  window.mvaddstr(2, 0, "Developers:");

  curs_set(0);

  window.refresh();

  // set non-blocking mode
  //
  window.timeout(100);
  window.keypad(true);
  noecho();

  //let mut quitting : bool = false;
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

      //if quitting == true {
      //  window.mvaddstr(window.get_max_y() - 1 , 0, "CLOSING");
      //} else {
        
        let mut owned_string: String = "CMD> ".to_owned();
        owned_string.push_str(&command_string);

        window.mvaddstr( window.get_max_y() - 1 , 0, owned_string);
      //}
      
      window.refresh();

  }

  endwin();
}