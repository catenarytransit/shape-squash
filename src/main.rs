use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct RawShape {
    pub shape_id: String,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: usize,
    pub shape_dist_traveled: Option<f32>,
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
    
    let mut wtr = csv::Writer::from_path(&output_file_path).unwrap();

    for result in rdr.deserialize() {
        if let Ok(record) = result {
            let record: RawShape = record;

            if current_shape_id.as_ref().is_none() {
                current_shape_id = Some(record.shape_id.clone());
            }

            //if the shape id is different, write the previous shape to file

            if current_shape_id.as_ref() != Some(&record.shape_id) {
                // Write the previous shape to file

                for shape in &shapes {
                    wtr.serialize(shape).unwrap();
                }

                // Clear the shapes vector and update the current shape id
                shapes.clear();
                current_shape_id = Some(record.shape_id.clone());
            }

            // If the shape id is the same, just push the record to the shapes vector
            if shapes.is_empty() {
                shapes.push(record);
            } else {
                let last = shapes.last().unwrap();

                if last.shape_pt_lat == record.shape_pt_lat && last.shape_pt_lon == record.shape_pt_lon {
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

    for shape in &shapes {
        wtr.serialize(shape).unwrap();
    }
    shapes.clear();

    // rename output back to input

    std::fs::rename(output_file_path, file_path).unwrap();
}
