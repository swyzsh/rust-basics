#![allow(dead_code)]

enum Stage {
  Beginner,
  Advanced,
}

enum Role {
  Student,
  Teacher,
}

fn main() {
  // Explicitily `use` each name so they are available without manual scoping.
  use crate::Role::*;
  use crate::Stage::{Advanced, Beginner};

  // Equivalent to `Stage::Beginner`;
  let stage = Beginner;
  // Equivalent to `Role::Student`;
  let role = Student;

  match stage {
    // Note the lack of scoping because of the explicit `use` above.
    Beginner => println!("Beginners are starting their learning journey!"),
    Advanced => println!("Advanced learners are mastering their subjects"),
  }

  match role {
    // Note again the lack of scoping (eg. Role::Student) .
    Student => println!("Students are acquiring knowledge"),
    Teacher => println!("Teachers are spreading knowledge"),
  }
}
