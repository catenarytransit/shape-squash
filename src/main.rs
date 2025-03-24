use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct RawShape {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub sequence: usize,
    pub dist_traveled: Option<f32>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a file path as a command-line argument.");
        return;
    }

    let file_path = &args[1];

    let file = File::open(file_path).expect(&format!("Unable to open file: {}", file_path));
    let mut rdr = csv::Reader::from_reader(file);

    let output_file_path = format!("{}.temp", file_path);

    let mut shapes: Vec<RawShape> = Vec::new();

    let mut current_shape_id: Option<String> = None;

    for result in rdr.deserialize() {
        if let Ok(record) = result {
            let record: RawShape = record;

            if current_shape_id.as_ref().is_none() {
                current_shape_id = Some(record.id.clone());
            }

            //if the shape id is different, write the previous shape to file

            if current_shape_id.as_ref() != Some(&record.id) {
                // Write the previous shape to file
                let mut wtr = csv::Writer::from_path(&output_file_path).unwrap();

                for shape in &shapes {
                    wtr.serialize(shape).unwrap();
                }
                wtr.flush().unwrap();

                // Clear the shapes vector and update the current shape id
                shapes.clear();
                current_shape_id = Some(record.id.clone());
            }

            // If the shape id is the same, just push the record to the shapes vector
            if shapes.is_empty() {
                shapes.push(record);
            } else {
                let last = shapes.last().unwrap();

                if last.latitude == record.latitude && last.longitude == record.longitude {
                    //do nothing
                } else {
                    shapes.push(record);
                }
            }
        } else {
            eprintln!("Error reading record");
        }
    }

    // Write the last shape to file after the loop

    let mut wtr = csv::Writer::from_path(&output_file_path).unwrap();
    for shape in &shapes {
        wtr.serialize(shape).unwrap();
    }
    shapes.clear();
    wtr.flush().unwrap();

    // rename output back to input

    std::fs::rename(output_file_path, file_path).unwrap();
}
