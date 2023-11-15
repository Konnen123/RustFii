use std::{fs, io, process::exit};

use rusqlite::{Connection, ErrorCode};
const ARGUMENT_SIZE: usize = 10;
trait CommandFunctions {
    fn get_name(&self) -> String;
    fn exec(&mut self, arguments: &[&str]);
}
struct Bookmark {
    name: String,
    url: String,
}
struct Terminal {
    commands: Vec<Box<dyn CommandFunctions>>,
}
impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: Vec::new(),
        }
    }
    fn register(&mut self, command: Box<dyn CommandFunctions>) {
        self.commands.push(command);
    }
    fn run(&mut self) {
        'outer: loop {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).expect("Error at read line");
            let mut args = [""; ARGUMENT_SIZE];
            let mut iter = buf.split_whitespace();
            for i in 0..ARGUMENT_SIZE {
                match iter.next() {
                    Some(parameter) => {
                        args[i] = parameter;
                    }
                    None => break,
                }
            }

            for i in 0..self.commands.len() {
                let command: &mut Box<dyn CommandFunctions> = &mut self.commands[i];
                if command.get_name() == args[0] {
                    command.exec(&args);
                    continue 'outer;
                }
            }

            match args[0].to_lowercase().as_str() {
                "pin" | "ping" => {
                    println!("Command not found! Did you want to type ping?");
                }
                "tim" | "times" | "time" => {
                    println!("Command not found! Did you want to type times?");
                }
                "c" | "cp" => {
                    println!("Command not found! Did you want to type cp?");
                }
                "st" | "stop" => {
                    println!("Command not found! Did you want to type stop?");
                }
                "b" | "bm" => {
                    println!("Command not found! Did you want to type bm?");
                }
                _ => {
                    println!("Command not found!");
                }
            }
        }
    }
}
struct PingCommand;
struct TimesCommand {
    count: i32,
}
struct CpCommand;
struct CountCommand;
struct BmCommand;
struct StopCommand;

impl CommandFunctions for PingCommand {
    fn get_name(&self) -> String {
        String::from("ping")
    }

    fn exec(&mut self, arguments: &[&str]) {
        println!("pong!");
    }
}
impl CommandFunctions for TimesCommand {
    fn get_name(&self) -> String {
        String::from("times")
    }

    fn exec(&mut self, arguments: &[&str]) {
        if arguments.len() < 2 {
            println!("Error! Times command could not be run because counter has not been passed!");
            return;
        }
        self.count += 1;
        println!("This command has been called for {} times", self.count);
    }
}
impl CommandFunctions for CpCommand {
    fn get_name(&self) -> String {
        String::from("cp")
    }

    fn exec(&mut self, arguments: &[&str]) {
        if arguments.len() < 3 {
            println!("Error! Cp command could not be run! Too few arguments!");
            return;
        }
        if arguments[1].is_empty() || arguments[2].is_empty() {
            println!("Error! Arg[1] or arg[2] is empty!");
            return;
        }
        match fs::read(arguments[1]) {
            Ok(data) => match fs::write(arguments[2], data) {
                Ok(ok) => {
                    println!("Cp command was successful!")
                }
                Err(error) => {
                    println!("Error at writing to arg[2] {error}")
                }
            },
            Err(error) => {
                println!("Error at reading the file from arg[1]!{error}");
            }
        }
    }
}
impl CommandFunctions for CountCommand {
    fn get_name(&self) -> String {
        String::from("count")
    }

    fn exec(&mut self, arguments: &[&str]) {
        let mut count = 0;
        for i in 1..arguments.len() {
            if arguments[i] != "" {
                count += 1;
            }
        }

        println!("Counted {} arguments!", count);
    }
}
impl CommandFunctions for BmCommand {
    fn get_name(&self) -> String {
        String::from("bm")
    }
    fn exec(&mut self, arguments: &[&str]) {
        match arguments[1] {
            "add" => {
                if arguments[2] == "" || arguments[3] == "" {
                    println!("Arguments name or url are empty!");
                    return;
                }
                let connectionResult = Connection::open("bookmark.db");
                match connectionResult {
                    Ok(connection) => {
                        let create = r"
                                create table if not exists bookmarks (
                                name text    not null,
                                url  text    not null
                                );
                                ";
                        match connection.execute(create, ()) {
                            Ok(_size) => {
                                match connection.execute(
                                    "insert into bookmarks (name, url) values (?1, ?2);",
                                    (arguments[2], arguments[3]),
                                ) {
                                    Ok(_size) => {
                                        println!("Added {} into bookmark database", arguments[2])
                                    }
                                    Err(error) => println!(
                                        "Error at adding {} into bookmark database. Error: {}",
                                        arguments[2], error
                                    ),
                                }
                            }
                            Err(error) => {
                                println!("Error at connecting to database: {error}");
                            }
                        }
                    }
                    Err(error) => println!("Error at opening the bookmark database:{error}"),
                }
            }
            "search" => {
                if arguments[2] == "" {
                    println!("Arguments name are empty!");
                    return;
                }
                let connectionResult = Connection::open("bookmark.db");
                match connectionResult {
                    Ok(connection) => match connection.prepare("select * from bookmarks") {
                        Ok(mut stmt) => {
                            match stmt.query_map([], |row| {
                                Ok(Bookmark {
                                    name: row.get("name")?,
                                    url: row.get("url")?,
                                })
                            }) {
                                Ok(bookmark_iter) => {
                                    for i in bookmark_iter {
                                        match i {
                                            Ok(bookmark) => {
                                                if bookmark.name.contains(arguments[2]) {
                                                    println!("Name: {}", bookmark.name);
                                                }
                                                if bookmark.url.contains(arguments[2]) {
                                                    println!("Url: {}", bookmark.url);
                                                }
                                            }
                                            Err(error) => {
                                                println!("Error at parsing from bookmark: {error}")
                                            }
                                        }
                                    }
                                }
                                Err(error) => println!("Error at query_map: {error}"),
                            }
                        }
                        Err(error) => println!("Error at selecting from bookmark: {error}"),
                    },
                    Err(error) => println!("Error at opening the bookmark database:{error}"),
                }
            }
            _ => {
                println!("Command not found for bm. Available commands are add and search");
            }
        }
    }
}
impl CommandFunctions for StopCommand {
    fn get_name(&self) -> String {
        String::from("stop")
    }

    fn exec(&mut self, arguments: &[&str]) {
        exit(0);
    }
}
fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(CpCommand {}));
    terminal.register(Box::new(BmCommand {}));
    terminal.register(Box::new(StopCommand {}));

    terminal.run();
}
