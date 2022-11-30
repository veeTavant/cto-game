
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Architecture {
    ProofofConcept,
    Monolith,
    Microservices,
    EventDriven
}

pub struct Software {
    _lines_of_code:         u32,    // total
    _age_of_code:           u16,    // weeks
    _complexity_of_code:    u16,    // 0 - 100
    _feature_richness:      u16,    // 0 - 100
    _ease_of_use:           u16,    // 0 - 100
    _components:            u16,    // components 1 - 100
    _services:              u16,    // services 1 - 100
    _dependencies:          u16,    // dependencies 1 - 100
    _cost_of_service:       u16,    // price of service
    _architecture: Architecture,    // what is the predominant architecture
    _customer_satisfaction: u16,    // 0 - 100
    _customers: u16,                // number of
    _active_users: u16              // number of
}


// Active users can be way more than customers depending on model (B2B or B2C etc)
//
impl Software {

    pub fn new(lines_of_code: u32, age_of_code: u16, complexity_of_code: u16, cost_of_service: u16) -> Software {   
        return Software { _lines_of_code: lines_of_code,
                          _age_of_code: age_of_code,
                          _complexity_of_code: complexity_of_code,
                          _feature_richness: 0,
                          _ease_of_use: 0,
                          _components: 0,
                          _services: 0,
                          _dependencies: 0,
                          _cost_of_service: cost_of_service,
                          _architecture: Architecture::ProofofConcept,
                          _customer_satisfaction: 0,
                          _customers: 0,
                          _active_users: 0
                        };
    }

    pub fn lines_of_code(&self) -> u32 {
        self._lines_of_code
    }

    pub fn age_of_code(&self) -> u16 {
        self._age_of_code
    }

    pub fn complexity_of_code(&self) -> u16 {
        self._complexity_of_code
    }

    pub fn cost_of_service(&self) -> u16 {
        self._cost_of_service
    }

    pub fn add_lines(&mut self, lines :u32) {
        self._lines_of_code += lines
    }

    pub fn remove_lines(&mut self, lines: u32) {
        self._lines_of_code -= lines
    }

    pub fn add_age(&mut self, age :u16) {
        self._age_of_code += age
    }

    pub fn add_complexity(&mut self, complexity :u16) {
        self._complexity_of_code += complexity
    }

    pub fn remove_complexity(&mut self, complexity :u16) {
        self._complexity_of_code -= complexity
    }

    pub fn get_architecture(&self) -> Architecture {
        self._architecture
    }

    pub fn customers(&self) -> u16 {
        self._customers
    }

    pub fn active_users(&self) -> u16 {
        self._active_users
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