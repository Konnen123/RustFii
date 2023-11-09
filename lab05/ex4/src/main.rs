use std::{fs, path::StripPrefixError};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Student
{
    name:String,
    phone:String,
    age:u8
}
fn main() 
{
    get_youngest_and_oldest_person();
}

fn get_youngest_and_oldest_person()
{
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

    let content = fs::read_to_string("src/data.json");
    match content
    {
        Ok(content)=>
        {
            let lines :Vec<&str> = content.lines().collect();
            for line in lines
            {
                let student : Student = serde_json::from_str(line).unwrap();
                if student.age<youngest_student.age
                {
                    youngest_student=student;
                }
                else if student.age>oldest_student.age
                {
                    oldest_student=student;
                }
                
            }
        }
        Err(error)=>println!("Error at reading from json file : {}",error),
    }

    println!("Youngest is {:?} and the oldest is {:?}",youngest_student,oldest_student);
}