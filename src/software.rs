

pub struct Software {
    pub lines_of_code: u32,        // total
    pub age_of_code: u32,          // months
    pub complexity_of_code: u32    // 0-1000
}

impl Software {
    fn add_lines(&mut self, lines :u32) {
        self.lines_of_code += lines
    }

    fn remove_lines(&mut self, lines: u32) {
        self.lines_of_code -= lines
    }

    fn add_age(&mut self, age :u32) {
        self.age_of_code += age
    }

    fn add_complexity(&mut self, complexity :u32) {
        self.complexity_of_code += complexity
    }

    fn remove_complexity(&mut self, complexity :u32) {
        self.complexity_of_code -= complexity
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn software_lines_tests() {

        let mut software = Software { lines_of_code: 0, age_of_code: 0, complexity_of_code: 0 };

        software.add_lines(100);
        assert_eq!(software.lines_of_code, 100);

        software.remove_lines(50);
        assert_eq!(software.lines_of_code, 50);
    }

    #[test]
    fn software_age_tests() {

        let mut software = Software { lines_of_code: 0, age_of_code: 0, complexity_of_code: 0 };

        software.add_age(100);
        assert_eq!(software.age_of_code, 100);
    }

    #[test]
    fn software_complexity_tests() {

        let mut software = Software { lines_of_code: 0, age_of_code: 0, complexity_of_code: 0 };

        software.add_complexity(100);
        assert_eq!(software.complexity_of_code, 100);

        software.remove_complexity(50);
        assert_eq!(software.complexity_of_code, 50);
    }
}