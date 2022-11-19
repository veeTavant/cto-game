
trait Grow {
    fn grow(&mut self);
}

pub struct Software {
    pub lines_of_code: u32,        // total
    pub age_of_code: u32,          // months
    pub complexity_of_code: u32    // 0-1000
}

impl Grow for Software {
    fn grow(&mut self) {

    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn software_tests() {

        let mut software = Software { lines_of_code: 0, age_of_code: 0, complexity_of_code: 0 };

        software.grow();

        assert_eq!(software.lines_of_code, 0);
    }
}