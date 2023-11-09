use std::{fs, io};

#[derive(Debug)]
struct Student {
    name: String,
    phone: String,
    age: u8,
}

fn main() {
    match check_youngest_and_oldest() {
        Ok(students) => println!(
            "Youngest student is: {:?}. \nOldest student is: {:?}",
            students.0, students.1
        ),
        Err(error) => println!("error: {}", error),
    }
}
fn check_youngest_and_oldest() -> Result<(Student, Student), io::Error> {
    let mut youngest_student: Student = Student {
        name: String::from(""),
        phone: String::from(""),
        age: u8::MAX,
    };
    let mut oldest_student: Student = Student {
        name: String::from(""),
        phone: String::from(""),
        age: u8::MIN,
    };

    let data: String = fs::read_to_string("src/studentProperties.txt")?;

    let lines: Vec<&str> = data.split('\n').collect();
    for line in lines {
        let split_data: Vec<&str> = line.split(',').collect();
        match split_data[2].trim().parse::<u8>() {
            Ok(age) => {
                if age > oldest_student.age {
                    oldest_student.age = age;
                    oldest_student.phone = split_data[1].to_string();
                    oldest_student.name = split_data[0].to_string();
                } else if age < youngest_student.age {
                    youngest_student.age = age;
                    youngest_student.phone = split_data[1].to_string();
                    youngest_student.name = split_data[0].to_string();
                }
            }
            Err(error) => println!("error at parsing the age {}", error),
        }
    }

    Ok((youngest_student, oldest_student))
}
