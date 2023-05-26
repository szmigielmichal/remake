use std::fs::{File, self};
use std::io::{ self, BufRead, BufReader, Write };
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?; 
    Ok(io::BufReader::new(file).lines())
}

fn process_file(joined_data: String, mut file: File) -> Result<(), std::io::Error> {
    file.write_all(joined_data.as_bytes())?;
    fs::remove_file("./Makefile")?;
    fs::rename("Makefile.tmp", "Makefile")?;

    Ok(())
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
    if let Ok(lines) = read_lines("./Makefile".to_string()) {
        let file = File::create("Makefile.tmp")?;

        let mut data = vec![];
        
        for line in lines {
            data.push(replace(line?));
        }

        process_file(data.join("\n"), file)?;
    };
    Ok(())
}

