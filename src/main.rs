use clap::Parser;
use fast_float;
use memchr::memchr;
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;
use std::io::Read;
use std::time::Instant;

const READ_BUF_SIZE: usize = 128 * 1024; // 128 KiB

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

fn process_chunk(data: &[u8], result: &mut FxHashMap<Box<[u8]>, StationValues>) -> () {
    let mut buffer = &data[..];

    loop {
        match memchr(b';', &buffer) {
            None => {
                break;
            }
            Some(comma_seperator) => {
                let end = memchr(b'\n', &buffer[comma_seperator..]).unwrap();
                let name = &buffer[..comma_seperator];
                let value = &buffer[comma_seperator + 1..comma_seperator + end];
                let value: f32 = fast_float::parse(value).expect("Failed to parse value");

                result
                    .entry(name.into())
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
                buffer = &buffer[comma_seperator + end + 1..];
            }
        }
    }

    // result
}

pub fn round_off(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}

fn write_result_stdout(mut result: FxHashMap<Box<[u8]>, StationValues>) -> () {
    let mut ordered_result = BTreeMap::new();
    for (station_name, station_values) in result.iter_mut() {
        ordered_result.insert(station_name, station_values);
    }
    let mut iterator = ordered_result.iter().peekable();
    print!("{{");
    while let Some((station_name, station_values)) = iterator.next() {
        if iterator.peek().is_none() {
            print!(
                "{}={:.1}/{:.1}/{:.1}}}",
                std::str::from_utf8(station_name)
                    .expect("Unable to validate station name as UTF-8"),
                station_values.min,
                station_values.mean,
                station_values.max
            );
        } else {
            print!(
                "{}={:.1}/{:.1}/{:.1}, ",
                std::str::from_utf8(station_name)
                    .expect("Unable to validate station name as UTF-8"),
                station_values.min,
                station_values.mean,
                station_values.max
            );
        }
    }
}

fn calculate_station_values(mut file: std::fs::File) -> FxHashMap<Box<[u8]>, StationValues> {
    // Start the processor threads
    let (sender, receiver) = crossbeam_channel::bounded::<Box<[u8]>>(1_000);
    let n_threads = std::thread::available_parallelism().unwrap().into();
    let mut handles = Vec::with_capacity(n_threads);
    for _ in 0..n_threads {
        let receiver = receiver.clone();
        let handle = std::thread::spawn(move || {
            let mut result = FxHashMap::<Box<[u8]>, StationValues>::default();
            // wait until the sender sends the chunk
            for buf in receiver {
                process_chunk(&buf, &mut result);
            }
            result
        });
        handles.push(handle);
    }

    // Read the file in chunks and send the chunks to the processor threads
    let mut buf = vec![0; READ_BUF_SIZE];
    let mut unprocessed_buffer: Vec<u8> = Vec::new();
    loop {
        let bytes_read = file.read(&mut buf[..]).expect("Failed to read file");
        // println!("bytes_Read {:?}", bytes_read);
        if bytes_read == 0 {
            break;
        }

        let actual_buf = &mut buf[..bytes_read];
        let last_new_line_index = match find_new_line_pos(&actual_buf) {
            Some(index) => index,
            None => {
                // No newline found in the buffer. Store all the bytes in unprocessed_buffer
                // and continue reading the file
                // TODO: handle this case
                unprocessed_buffer.append(&mut actual_buf.to_owned());
                continue;
            }
        };

        if bytes_read == last_new_line_index + 1 {
            // If the buffer is full, then we can safely assume that the last byte is a newline
            // and we can process the buffer

            if unprocessed_buffer.len() != 0 {
                unprocessed_buffer.append(&mut actual_buf[..(last_new_line_index + 1)].to_owned());
                let buf_boxed = Box::<[u8]>::from(&unprocessed_buffer[..]);
                sender.send(buf_boxed).expect("Failed to send buffer");
                unprocessed_buffer.clear();
            } else {
                let buf_boxed = Box::<[u8]>::from(&actual_buf[..(last_new_line_index + 1)]);
                sender.send(buf_boxed).expect("Failed to send buffer");
            }
        } else {
            // If the buffer is not full, then we can't assume that the last byte is a newline
            // We need to store the bytes that are not processed in unprocessed_buffer
            // and continue reading the file

            // Send chunk till last new line
            if unprocessed_buffer.len() != 0 {
                unprocessed_buffer.append(&mut actual_buf[..(last_new_line_index + 1)].to_owned());
                let buf_boxed = Box::<[u8]>::from(&unprocessed_buffer[..]);
                sender.send(buf_boxed).expect("Failed to send buffer");
                unprocessed_buffer.clear();
                unprocessed_buffer.append(&mut actual_buf[(last_new_line_index + 1)..].to_vec());
            } else {
                let buf_boxed = Box::<[u8]>::from(&actual_buf[..(last_new_line_index + 1)]);
                sender.send(buf_boxed).expect("Failed to send buffer");
                unprocessed_buffer.append(&mut actual_buf[(last_new_line_index + 1)..].to_vec());
            }
        }
    }
    drop(sender);

    // Combine data from all threads
    let mut result = FxHashMap::<Box<[u8]>, StationValues>::default();
    for handle in handles {
        let map = handle.join().unwrap();
        for (station_name, station_values) in map.into_iter() {
            // dbg!(station_values);
            result
                .entry(station_name)
                .and_modify(|e| {
                    if station_values.min < e.min {
                        e.min = station_values.min;
                    }
                    if station_values.max > e.max {
                        e.max = station_values.max;
                    }
                    e.mean = e.mean + station_values.mean;
                    e.count += station_values.count;
                })
                .or_insert(station_values);
        }
    }

    // Calculate the mean for all entries and round off to 1 decimal place
    for (_name, station_values) in result.iter_mut() {
        station_values.mean = round_off(station_values.mean / station_values.count as f32);
        station_values.min = round_off(station_values.min);
        station_values.max = round_off(station_values.max);
    }

    return result
}

fn main() {
    let start = Instant::now();
    let args = Args::parse();

    let file = std::fs::File::open(&args.file).expect("Failed to open file");
    let result = calculate_station_values(file);
    write_result_stdout(result);

    let duration = start.elapsed();
    println!("\nTime taken is: {:?}", duration);
}

fn find_new_line_pos(bytes: &[u8]) -> Option<usize> {
    // In this case (position is not far enough),
    // naive version is faster than bstr (memchr)
    bytes.iter().rposition(|&b| b == b'\n')
}

#[cfg(test)]
mod tests {
    use crate::{StationValues, calculate_station_values};
    use std::{collections::HashMap, fs, path::PathBuf};

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

            let mut result = calculate_station_values(file);
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
