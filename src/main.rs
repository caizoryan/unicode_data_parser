use std::{collections::HashMap, fs, io::BufWriter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Record {
    script: String,
    count: f64,
    v: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RecordD {
    script: String,
    d6: f64,
    d7: f64,
    d8: f64,
    d9: f64,
    d10: f64,
    d11: f64,
    d12: f64,
    d13: f64,
    d14: f64,
    d15: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RecordP {
    script: String,
    p6: f64,
    p7: f64,
    p8: f64,
    p9: f64,
    p10: f64,
    p11: f64,
    p12: f64,
    p13: f64,
    p14: f64,
    p15: f64,
    pt: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RecordV {
    script: String,
    v5: f64,
    v6: f64,
    v7: f64,
    v8: f64,
    v9: f64,
    v10: f64,
    v11: f64,
    v12: f64,
    v13: f64,
    v14: f64,
    v15: f64,
}

fn main() {
    // We load in the file that we are working with
    load_and_write2()
}

fn load_and_write2() {
    let buf = Vec::new();
    let mut wtr = csv::Writer::from_writer(buf);

    let mut records = Vec::new();

    let paths = fs::read_dir("./files").unwrap();

    let mut sexy_map: HashMap<String, RecordV> = HashMap::new();

    for path in paths {
        let path = path.unwrap().path();
        let data = std::fs::read_to_string(path.clone()).expect("cannot read file");
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let version = file_name.replace(".txt", "").replace("Scripts", "");

        println!("Processing: {}", version);

        let map = create_map(data.clone());

        map.iter().for_each(|(script, count)| {
            let mut record = RecordV {
                script: script.to_string(),
                v5: 0.0,
                v6: 0.0,
                v7: 0.0,
                v8: 0.0,
                v9: 0.0,
                v10: 0.0,
                v11: 0.0,
                v12: 0.0,
                v13: 0.0,
                v14: 0.0,
                v15: 0.0,
            };

            match version.as_str() {
                "5" => record.v5 = *count,
                "6" => record.v6 = *count,
                "7" => record.v7 = *count,
                "8" => record.v8 = *count,
                "9" => record.v9 = *count,
                "10" => record.v10 = *count,
                "11" => record.v11 = *count,
                "12" => record.v12 = *count,
                "13" => record.v13 = *count,
                "14" => record.v14 = *count,
                "15" => record.v15 = *count,
                _ => {
                    println!("Unknown version: {}", version);
                }
            };

            sexy_map
                .entry(script.to_string())
                .and_modify(|rec| match version.as_str() {
                    "5" => rec.v5 = *count,
                    "6" => rec.v6 = *count,
                    "7" => rec.v7 = *count,
                    "8" => rec.v8 = *count,
                    "9" => rec.v9 = *count,
                    "10" => rec.v10 = *count,
                    "11" => rec.v11 = *count,
                    "12" => rec.v12 = *count,
                    "13" => rec.v13 = *count,
                    "14" => rec.v14 = *count,
                    "15" => rec.v15 = *count,
                    _ => {
                        println!("Unknown version: {}", version);
                    }
                })
                .or_insert(record);
        });
    }

    sexy_map.iter().for_each(|(_, record)| {
        records.push(record);
    });

    let mut records = transform_count_to_difference_in_percentage(records);

    records.sort_by(|records_a, records_b| records_b.pt.total_cmp(&records_a.pt));

    records.iter().enumerate().for_each(|(i, record)| {
        if i <= 10 {
            wtr.serialize(record).unwrap();
        }
    });

    wtr.flush().unwrap();

    let buf = wtr.into_inner().unwrap();
    let s = String::from_utf8(buf).unwrap();

    fs::write("./records_percentage_top_10_mean.csv", s).expect("cannot write file");
}

fn calculate_percentage(difference: f64, total: f64) -> f64 {
    match total {
        0.0 => 0.0,
        _ => {
            let calc = difference * 100.0 / total;
            if calc < f64::from(1) && difference > f64::from(0) {
                println!("{} / {} = {}", difference, total, calc);
            }
            calc
        }
    }
}

fn transform_count_to_difference_in_percentage(records: Vec<&RecordV>) -> Vec<RecordP> {
    let mut new_records = Vec::new();

    records.iter().for_each(|record| {
        let mut new_record = RecordP {
            script: record.script.to_string(),
            p6: 0.0,
            p7: 0.0,
            p8: 0.0,
            p9: 0.0,
            p10: 0.0,
            p11: 0.0,
            p12: 0.0,
            p13: 0.0,
            p14: 0.0,
            p15: 0.0,
            pt: 0.0,
        };

        new_record.p6 = calculate_percentage(record.v6 - record.v5, record.v5);
        new_record.p7 = calculate_percentage(record.v7 - record.v6, record.v6);
        new_record.p8 = calculate_percentage(record.v8 - record.v7, record.v7);
        new_record.p9 = calculate_percentage(record.v9 - record.v8, record.v8);
        new_record.p10 = calculate_percentage(record.v10 - record.v9, record.v9);
        new_record.p11 = calculate_percentage(record.v11 - record.v10, record.v10);
        new_record.p12 = calculate_percentage(record.v12 - record.v11, record.v11);
        new_record.p13 = calculate_percentage(record.v13 - record.v12, record.v12);
        new_record.p14 = calculate_percentage(record.v14 - record.v13, record.v13);
        new_record.p15 = calculate_percentage(record.v15 - record.v14, record.v14);

        let sum = new_record.p6
            + new_record.p7
            + new_record.p8
            + new_record.p9
            + new_record.p10
            + new_record.p11
            + new_record.p12
            + new_record.p13
            + new_record.p14
            + new_record.p15;

        new_record.pt = sum / 9.0;

        new_records.push(new_record);
    });

    new_records
}

fn transform_count_to_differnce(records: Vec<&RecordV>) -> Vec<RecordD> {
    let mut new_records = Vec::new();

    records.iter().for_each(|record| {
        let mut new_record = RecordD {
            script: record.script.to_string(),
            d6: 0.0,
            d7: 0.0,
            d8: 0.0,
            d9: 0.0,
            d10: 0.0,
            d11: 0.0,
            d12: 0.0,
            d13: 0.0,
            d14: 0.0,
            d15: 0.0,
        };

        new_record.d6 = record.v6 - record.v5;
        new_record.d7 = record.v7 - record.v6;
        new_record.d8 = record.v8 - record.v7;
        new_record.d9 = record.v9 - record.v8;
        new_record.d10 = record.v10 - record.v9;
        new_record.d11 = record.v11 - record.v10;
        new_record.d12 = record.v12 - record.v11;
        new_record.d13 = record.v13 - record.v12;
        new_record.d14 = record.v14 - record.v13;
        new_record.d15 = record.v15 - record.v14;

        new_records.push(new_record);
    });

    new_records
}

fn load_and_write() {
    let buf = Vec::new();
    let mut wtr = csv::Writer::from_writer(buf);

    let mut records = Vec::new();

    let paths = fs::read_dir("./files").unwrap();

    let mut sexy_map = HashMap::new();

    for path in paths {
        let path = path.unwrap().path();
        let data = std::fs::read_to_string(path.clone()).expect("cannot read file");
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let version = file_name.replace(".txt", "").replace("Scripts", "");

        let map = create_map(data.clone());

        sexy_map.insert(version, map);
    }

    sexy_map.iter().for_each(|(version, map)| {
        map.iter().for_each(|(script, count)| {
            let record = Record {
                script: script.to_string(),
                count: *count,
                v: version.parse::<f64>().unwrap(),
            };

            records.push(record);
        });
    });

    records.iter().for_each(|record| {
        wtr.serialize(record).unwrap();
    });

    wtr.flush().unwrap();

    let buf = wtr.into_inner().unwrap();
    let s = String::from_utf8(buf).unwrap();

    fs::write("./files/records.csv", s).expect("cannot write file");
}

fn create_map(data: String) -> HashMap<String, f64> {
    // We split the file into lines
    let lines: Vec<&str> = data.lines().collect();

    // We filter out the lines that start with a #
    let lines_without_comments = lines
        .iter()
        .filter(|line| !line.starts_with("#"))
        .collect::<Vec<_>>();

    // Now seperate each of the line that we are concerned with using a semicolon
    // This will give us a vector of vectors
    //
    //
    // # Example:
    // [[hexadecimal_value, name_of_script], [hexadecimal_value, name_of_script] .... so on]
    //
    //
    let rows = lines_without_comments
        .iter()
        .map(|line| line.split(";").collect::<Vec<_>>())
        // We filter out the lines that do not have 2 elements
        // So empty lines and lines that do not have a semicolon
        .filter(|row| row.len() == 2)
        .collect::<Vec<_>>();

    // remove the comments from the rows
    let rows = rows
        .iter()
        .map(|row| {
            let mut row = row.to_vec();
            row[1] = row[1].split("#").collect::<Vec<_>>()[0];
            row
        })
        .collect::<Vec<_>>();

    // if the first element in the row is a range (includes ..) then we calculate the range and set
    // count to it
    // else the count is 1
    //
    let mut new_row = Vec::new();

    rows.iter().for_each(|row| {
        let mut count = 1.0;
        let mut row = row
            .to_vec()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        if row[0].contains("..") {
            let range = row[0].split("..").collect::<Vec<_>>();
            let r1 = i32::from_str_radix(range[0].trim(), 16);
            let r2 = i32::from_str_radix(range[1].trim(), 16);
            count = f64::from(r2.unwrap() - r1.unwrap() + 1);
        }

        let count = count.to_string();

        row.push(count);
        new_row.push(row);
    });

    // flatten the data by using the hasmap
    let mut flattened_data = std::collections::HashMap::new();

    new_row.iter().for_each(|row| {
        let count = row[2].parse::<f64>().unwrap();
        let script = row[1].trim().to_string();
        flattened_data
            .entry(script)
            .and_modify(|val| *val += count)
            .or_insert(count);
    });

    flattened_data
}
