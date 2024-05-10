use clap::Parser;
use std::collections::BTreeMap;
// use std::str::from_utf8;
use std::time::Instant;
use fast_float;
use rustc_hash::FxHashMap;
use memmap2::Mmap;
use memchr::memchr;

#[derive(Parser, Debug)]
#[command(
    name = "rs-1brc",
    version = "1.0",
    about = "confusedHooman's version of 1BRC challenge"
)]
struct Args {
    #[arg(short = 'f', long, help = "Path to the measurement file")]
    file: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct StationValues {
    min: f32,
    max: f32,
    mean: f32,
    count: u32,
}

// Calculate the station values
fn calculate_station_values(data:&[u8]) -> FxHashMap<&[u8], StationValues> {
    let mut result: FxHashMap<&[u8], StationValues> = FxHashMap::default();
    let  mut buffer = data;
    loop {
        match memchr(b';', &buffer) {
            None => {
                break;
            }
            Some(comma_seperator) => {
                let end = memchr(b'\n', &buffer[comma_seperator..]).unwrap();
                let name = &buffer[..comma_seperator];
                let value = &buffer[comma_seperator+1..comma_seperator+end];
                let value = fast_float::parse(value).expect("Failed to parse value");

                result
                    .entry(name)
                    .and_modify(|e| {
                        if value < e.min {
                            e.min = value;
                        }
                        if value > e.max {
                            e.max = value;
                        }
                        e.mean = e.mean + value;
                        e.count += 1;
                    })
                    .or_insert(StationValues {
                        min: value,
                        max: value,
                        mean: value,
                        count: 1,
                    });
                buffer = &buffer[comma_seperator+end+1..];
            }

        }
    }


    // Calculate the mean for all entries and round off to 1 decimal place
    for (_, station_values) in result.iter_mut() {
        station_values.mean = round_off(station_values.mean / station_values.count as f32);
        station_values.min = round_off(station_values.min);
        station_values.max = round_off(station_values.max);
    }

    result
}

fn round_off(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}

fn write_result_stdout(result: FxHashMap<&[u8], StationValues>) -> () {
    let mut ordered_result = BTreeMap::new();
    for (station_name, station_values) in result {
        ordered_result.insert(station_name, station_values);
    }
    let mut iterator = ordered_result.iter().peekable();
    print!("{{");
    while let Some((station_name, station_values)) = iterator.next() {
        if iterator.peek().is_none() {
            print!(
                "{}={:.1}/{:.1}/{:.1}}}",
                std::str::from_utf8(station_name).expect("Unable to validate station name as UTF-8"), station_values.min, station_values.mean, station_values.max
            );
        } else {
            print!(
                "{}={:.1}/{:.1}/{:.1}, ",
                std::str::from_utf8(station_name).expect("Unable to validate station name as UTF-8"), station_values.min, station_values.mean, station_values.max
            );
        }
    }
}

fn main() {
    let start = Instant::now();
    let args = Args::parse();

    let file = std::fs::File::open(&args.file).expect("Failed to open file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to map file") };
    let data = &*mmap;

    let result = calculate_station_values(data);
    write_result_stdout(result);
    let duration = start.elapsed();
    println!("\nTime taken is: {:?}", duration);

}

#[cfg(test)]
mod tests {
    use crate::{calculate_station_values, StationValues};
    use std::{collections::HashMap, fs, path::PathBuf};
    use memmap2::Mmap;

    #[test]
    fn test_measurement_data() {
        let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let files = fs::read_dir(test_dir).unwrap();

        for file in files {
            let test_file_name = file.unwrap().path().to_str().unwrap().to_string();
            if test_file_name.ends_with(".out") {
                continue;
            }
            let output_file_name = test_file_name.replace(".txt", ".out");
            print!("\nTest file: {}\n", test_file_name);
            let test_output = read_test_output_file(output_file_name);

            let file = std::fs::File::open(test_file_name.clone()).expect("Failed to open file");
            let mmap = unsafe { Mmap::map(&file).expect("Failed to map file") };
            let data = &*mmap;
            let mut result = calculate_station_values(data);
            let mut test_output_map_copy = test_output.clone();

            // compare two hashmaps
            for (station_name, station_values) in test_output.into_iter() {
                let result_station_values = result.remove(station_name.as_bytes()).expect(
                    ("Station not found: ".to_string() + &station_name + " in result hashmap")
                        .as_str(),
                );
                assert_eq!(station_values.min, result_station_values.min);
                assert_eq!(station_values.max, result_station_values.max);
                assert_eq!(station_values.mean, result_station_values.mean);
                test_output_map_copy.remove(&station_name);
            }

            assert_eq!(result.len(), 0);
            assert_eq!(test_output_map_copy.len(), 0);

            print!("Test passed\n");
            print!("-----------------------------------\n");
        }
    }

    fn read_test_output_file(file_name: String) -> HashMap<String, StationValues> {
        let data = std::fs::read_to_string(file_name).expect("Failed to read file");
        // remove whitespace and braces
        // {Adelaide=15.0/15.0/15.0, Cabo San Lucas=14.9/14.9/14.9} => Adelaide=15.0/15.0/15.0, Cabo San Lucas=14.9/14.9/14.9
        let data_without_braces = data
            .trim_start()
            .trim_end()
            .trim_matches(['{', '}'].as_ref());

        // split the data by comma
        // Adelaide=15.0/15.0/15.0, Cabo San Lucas=14.9/14.9/14.9 => [Adelaide=15.0/15.0/15.0, Cabo San Lucas=14.9/14.9/14.9]
        let stations_data: Vec<&str> = data_without_braces.split(",").collect();
        let mut result: HashMap<String, StationValues> = HashMap::new();
        // split the data by "=" and "/" to get the station name and values
        for station_data in stations_data {
            let parts: Vec<&str> = station_data.split("=").collect();
            let station_name = parts[0].trim_start().trim_end().to_string();
            let values: Vec<&str> = parts[1].split("/").collect();
            let min = values[0].parse::<f32>().expect("Failed to parse min");
            let mean = values[1].parse::<f32>().expect("Failed to parse max");
            let max = values[2].parse::<f32>().expect("Failed to parse mean");
            result.insert(
                station_name,
                StationValues {
                    min,
                    max,
                    mean,
                    count: 0,
                },
            );
        }
        result
    }
}
