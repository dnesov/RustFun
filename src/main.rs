#![allow(dead_code)]

extern crate rand;
use rand::Rng;

const NAMES: [&'static str; 15] = [
    "John", "Daniel", "Mark", "Jonathan", "Jason", "Michael", "Madeline", "Rudolf", "Lester",
    "Shawn", "Tom", "Adrian", "Kevin", "Ralph", "Connor",
];

const SURNAMES: [&'static str; 16] = [
    "Peck",
    "Mcconnell",
    "Kelley",
    "Macias",
    "Welch",
    "Hurley",
    "Hartman",
    "Dorsey",
    "Villegas",
    "Ochoa",
    "Hamilton",
    "Robinson",
    "Gray",
    "Greenwood",
    "Robinson",
    "Walker",
];

pub struct Student {
    name: String,
    class_id: String,
    marks: Vec<i8>,
}

impl Student {
    pub fn new(name: String, class_id: String, marks: Vec<i8>) -> Self {
        return Student {
            name: name,
            class_id: class_id,
            marks: marks,
        };
    }
    fn pretty_print(&self) {
        println!(
            "Student's name: {}\nStudent's class: {}\nStudent's average: {}",
            self.name,
            self.class_id,
            self.average_mark()
        );
    }
    fn average_mark(&self) -> f32 {
        let avg: f32;

        let total: i8 = self.marks.iter().sum();
        avg = total as f32 / self.marks.len() as f32;

        return format!("{:.1$}", avg, 2).parse().unwrap();
    }
}

pub struct School {
    name: String,
    students: Vec<Student>,
}

impl School {
    pub fn new(name: String, students: Vec<Student>) -> School {
        return School {
            name: name,
            students: students,
        };
    }

    fn generate_students(&self, amount: i32) -> Vec<Student> {
        let mut students: Vec<Student> = vec![];

        let mut rng = rand::thread_rng();

        for i in 1..amount {
            let random_name: String = String::from(NAMES[rng.gen_range(0, NAMES.len())]);
            let random_surname: String = String::from(SURNAMES[rng.gen_range(0, SURNAMES.len())]);
            let mut marks: Vec<i8> = Vec::new();

            for i in 0..rng.gen_range(1, 10) {
                marks.push(rng.gen_range(1, 6));
            }

            students.push(Student {
                name: format!("{} {}", random_name, random_surname),
                class_id: String::from(rng.gen_range(1, 5).to_string()),
                marks: marks,
            })
        }
        return students;
    }

    fn print_all_students(&self) {
        for student in self.students.iter() {
            student.pretty_print();
        }
    }
}

fn main() {
    let mut school: School = School::new(String::from("xD"), vec![]);

    school.students = school.generate_students(12);
    school.print_all_students();
}
