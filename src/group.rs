use std::fmt;

use crate::Persons;

#[derive(Clone, Debug)]
pub struct Group {
    //pub edges: LinkedList<String>,
    pub group_size: i32,
    pub persons_in_group: Vec<Persons>,
}

impl Group {
    /*fn add_to_group(&mut self, p: Persons) {
        self.persons_in_group.fill(p);
    }*/

    /*fn clone(&self) -> Self {
        *self
    }*/

    fn copy(&mut self,group: Group) -> Group {
        let new_persons = self.persons_in_group.clone();
        let back = Group {
            group_size: self.group_size,
            persons_in_group: new_persons,
        };
        back
    }
}
impl fmt::Display for Group {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let t = self.persons_in_group.clone();
        for person in t {
            println!("{}", person.name);
            //write!(f, "{}", person.name);
        }
        write!(f, "{}", self.group_size)
    }
}
