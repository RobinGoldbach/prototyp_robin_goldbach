use std::marker::Copy;

use std::fmt;

#[derive(Debug, Clone,  PartialEq, Eq)]
pub struct Persons {
    pub name: i32,
    //pub name: &'static str,
    pub number_of_meetings: Vec<i32>,
    //pub peoplemet: [&'static str; 300],
    pub peoplemet:Vec<i32>,
}

impl Persons {
    /*fn add_people(&mut self, person: Persons, count: i32) {
        if (self.peoplemet.contains(&person.name)) {
            if (self.peoplemet.contains(&person.name)) {}
        } else {
            self.peoplemet.fill(person.name);
            self.number_of_meetings.fill(1);
        }
    }*/

    /*fn clone(&self) -> Self {
        *self
    }*/

    fn Copy(&mut self) -> Persons {
        let new_number_of_meetings = self.number_of_meetings.clone();
        let  back = Persons {
            peoplemet: self.peoplemet.clone(),
            name: self.name,
            number_of_meetings: new_number_of_meetings,
        };
        back
    }
}

impl fmt::Display for Persons {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.name)
    }
}
