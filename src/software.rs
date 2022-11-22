
pub struct Software {
    _lines_of_code:         u32,    // total
    _age_of_code:           u32,    // months
    _complexity_of_code:    u32,    // 0-1000
    _price_of_service:      u32     // price of service
}

impl Software {

    pub fn new(lines_of_code: u32, age_of_code: u32, complexity_of_code: u32, price_of_service: u32) -> Software {   
        return Software { _lines_of_code: lines_of_code, _age_of_code: age_of_code, _complexity_of_code: complexity_of_code, _price_of_service: price_of_service };
    }

    pub fn lines_of_code(&self) -> u32 {
        self.lines_of_code()
    }

    pub fn age_of_code(&self) -> u32 {
        self._age_of_code
    }

    pub fn complexity_of_code(&self) -> u32 {
        self._complexity_of_code
    }

    fn price_of_service(&self) -> u32 {
        self._price_of_service
    }

    fn add_lines(&mut self, lines :u32) {
        self._lines_of_code += lines
    }

    fn remove_lines(&mut self, lines: u32) {
        self._lines_of_code -= lines
    }

    fn add_age(&mut self, age :u32) {
        self._age_of_code += age
    }

    fn add_complexity(&mut self, complexity :u32) {
        self._complexity_of_code += complexity
    }

    fn remove_complexity(&mut self, complexity :u32) {
        self._complexity_of_code -= complexity
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn software_lines_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_lines(100);
        assert_eq!(software.lines_of_code(), 100);

        software.remove_lines(50);
        assert_eq!(software.lines_of_code(), 50);
    }

    #[test]
    fn software_age_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_age(100);
        assert_eq!(software._age_of_code, 100);
    }

    #[test]
    fn software_complexity_tests() {

        let mut software = Software::new(0, 0, 0, 0);

        software.add_complexity(100);
        assert_eq!(software._complexity_of_code, 100);

        software.remove_complexity(50);
        assert_eq!(software._complexity_of_code, 50);
    }
}