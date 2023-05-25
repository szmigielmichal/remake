use std::fs::{File, self};
use std::io::{ self, BufRead, BufReader, Write };

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn replace(line: &str) -> String {
    match &line {
        x if x.contains("PWD=$(shell pwd)") => "PWD=$(shell pwd) \nBRANCH=$(shell basename $(PWD))".to_string(),
        x if x.contains("NGINX_COMPOSE=-f docker-compose.yml -f") => "NGINX_COMPOSE=-f docker-compose.yml -f ${PWD}/dev/docker-compose.nginx.dev.yml".to_string(),
        x if x.contains("docker build -t local/$") => "\tdocker build -t local/${APP_NAME}-${BRANCH}:latest .".to_string(),
        x if x.contains("USER_UID=${UID} docker-compose ${API_COMPOSE} up -d --force-recreate") => "\tUSER_UID=${UID} IMAGE_NAME=local/${APP_NAME}-${BRANCH}:latest docker-compose ${API_COMPOSE} up -d --force-recreate".to_string(),
        _ => line.to_string()
    }
}
 
fn main() -> std::io::Result<()> {
    let lines = read_lines("./Makefile".to_string());
    let mut file = File::create("Makefile.tmp")?;

    let mut data = vec![];
    
    for line in lines {
        data.push(replace(line?.as_str()));
    }

    let joined_data = data.join("\n");

    file.write_all(joined_data.as_bytes())?;

    fs::remove_file("./Makefile")?;
    fs::rename("Makefile.tmp", "Makefile")?;

    Ok(())
}

