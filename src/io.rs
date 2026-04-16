use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write}; 
use std::path::Path;

pub const FILE_PATH_IN : &str= "../assets/";
pub const FILE_PATH_OUT : &str = "../output/";

pub const FILE_NAMES : &[&str; 10] = &[
    "wi29.tsp", "dj38.tsp", "qa194.tsp",
    "uy734.tsp", "zi929.tsp", "mu1979.tsp",
    "ca4663.tsp", "tz6117.tsp", "eg7146.tsp",
    "ei8246.tsp",
];

pub struct City {
    index : u16,
    x : f32,
    y : f32,
}

pub fn parse_tsp(file_path: &str) -> io::Result<Vec<City>> {
    let file = File::open(file_path)?; 
    let reader = BufReader::new(file);
    let mut cities = Vec::new();
    let mut is_coord_section = false;

    for line in reader.lines() {
        let line = line?; 
        let trimmed = line.trim();

        if trimmed.starts_with("NODE_COORD_SECTION") {
            is_coord_section = true;
            continue;
        }

        if trimmed == "EOF" || trimmed.starts_with("TOUR_SECTION") {
            break;
        }

        if is_coord_section {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            
            if parts.len() >= 3 {
                let index = parts[0].parse::<u16>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid index"))?;
                let x = parts[1].parse::<f32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid X coordinate"))?;
                let y = parts[2].parse::<f32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid Y coordinate"))?;

                cities.push(City { index, x, y });
            }
        }
    }

    Ok(cities)
}



pub fn save_tour(permutation: &[u16], file_path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;

    writeln!(file, "NAME: {}", file_path)?;
    writeln!(file, "TYPE: TOUR")?;
    writeln!(file, "DIMENSION: {}", permutation.len())?;
    writeln!(file, "TOUR_SECTION")?;

    for &index in permutation {
        writeln!(file, "{}", index)?;
    }

    writeln!(file, "-1")?;
    writeln!(file, "EOF")?;

    file.sync_all()?; 

    Ok(())
}
