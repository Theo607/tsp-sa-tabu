use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write}; 
use std::path::Path;

pub const FILE_PATH_IN : &str= "assets/";
pub const FILE_PATH_OUT : &str = "output/";

pub const FILE_NAMES : &[&str; 10] = &[
    "wi29.tsp", "dj38.tsp", "qa194.tsp",
    "uy734.tsp", "zi929.tsp", "mu1979.tsp",
    "ca4663.tsp", "tz6117.tsp", "eg7146.tsp",
    "ei8246.tsp",
];

pub struct City {
    pub index : u16,
    pub x : f32,
    pub y : f32,
}

pub fn parse_tsp(file_path: &str) -> io::Result<Vec<City>> {
    let file = File::open(file_path)?; 
    let reader = BufReader::new(file);
    let mut cities = Vec::new();
    let mut is_coord_section = false;
    let mut internal_index = 1;

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
                let x = parts[1].parse::<f32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid X"))?;
                let y = parts[2].parse::<f32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid Y"))?;

                cities.push(City { index: internal_index, x, y });
                internal_index += 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_mock_tsp(path: &str) {
        let content = "NAME: test\nTYPE: TSP\nNODE_COORD_SECTION\n1 10.0 20.0\n2 30.0 40.0\nEOF";
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_parse_tsp_valid_data() {
        let path = "test_valid.tsp";
        create_mock_tsp(path);

        let result = parse_tsp(path).expect("Should successfully parse");
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].index, 1);
        assert_eq!(result[0].x, 10.0);
        assert_eq!(result[1].y, 40.0);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_parse_tsp_missing_file() {
        let result = parse_tsp("non_existent_file.tsp");
        assert!(result.is_err());
    }

    #[test]
    fn test_save_tour_format() {
        let path = "test_output.tour";
        let route = vec![1, 2, 3];

        save_tour(&route, path).expect("Should save successfully");

        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("TYPE: TOUR"));
        assert!(content.contains("DIMENSION: 3"));
        assert!(content.contains("TOUR_SECTION"));
        assert!(content.contains("1\n2\n3\n-1"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_tour_creates_directory() {
        let path = "nested_dir/test.tour";
        let route = vec![1];

        save_tour(&route, path).expect("Should create dir and save");
        
        assert!(Path::new(path).exists());

        fs::remove_dir_all("nested_dir").unwrap();
    }
}
