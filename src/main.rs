use std::fs::{File, self};
use std::io::{ self, BufRead, BufReader, Write };
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>,
{
    match File::open(filename) {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => Err(error)
    }
}

fn process_file(joined_data: String, mut file: File) -> Result<(), std::io::Error> {
    match file.write_all(joined_data.as_bytes()) {
        Ok(_) => {
            fs::remove_file("./Makefile")?;
            fs::rename("Makefile.tmp", "Makefile")?;
            println!("Successfully wrote to Makefile");
            Ok(())
        },
        Err(why) => panic!("Couldn't write to Makefile {}", why)
    }
}

fn replace(line: String) -> String {
    match line {
        x if x.contains("PWD=$(shell pwd)") => "PWD=$(shell pwd)\nBRANCH=$(shell basename $(PWD))".to_string(),
        x if x.contains("NGINX_COMPOSE=-f docker-compose.yml -f") => "NGINX_COMPOSE=-f docker-compose.yml -f ${PWD}/dev/docker-compose.nginx.dev.yml".to_string(),
        x if x.contains("docker build -t local/$") => "\tdocker build -t local/${APP_NAME}-${BRANCH}:latest .".to_string(),
        x if x.contains("USER_UID=${UID} docker-compose ${API_COMPOSE} up -d --force-recreate") => "\tUSER_UID=${UID} IMAGE_NAME=local/${APP_NAME}-${BRANCH}:latest docker-compose ${API_COMPOSE} up -d --force-recreate".to_string(),
        _ => line
    }
}
 
fn main() -> std::io::Result<()> {
    match read_lines("./Makefile".to_string()) {
        Ok(lines) => {
            let file = File::create("Makefile.tmp")?;

            let mut data = vec![];
            
            for line in lines {
                data.push(replace(line?));
            }

            process_file(data.join("\n"), file)?;

            Ok(())
        },
        Err(err) => Err(err)
    }
}

