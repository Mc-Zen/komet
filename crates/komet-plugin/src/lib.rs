use std::vec;

use ciborium_io::Write as _;
use ciborium_ll::{Decoder, Encoder, Header};
use rustfft::{FftDirection, FftPlanner};
use wasm_minimal_protocol::*;
initiate_protocol!();

mod read;

#[wasm_func]
pub fn contour(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = Decoder::from(input);

    match decoder.pull().unwrap() {
        Header::Array(Some(len)) => {
            if len != 4 {
                return Err(String::from("Expected array of 4 elements, got more"));
            }

            let x = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len)?,
                _ => return Err(String::from("Bad input")),
            };

            let y = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len)?,
                _ => return Err(String::from("Bad input")),
            };

            let z = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len)?,
                _ => return Err(String::from("Bad input")),
            };

            let levels = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len)?,
                _ => return Err(String::from("Bad input")),
            };

            let contours = komet::contour(&x, &y, &z, &levels);

            let mut output = Vec::<u8>::new();
            let mut encoder = Encoder::from(&mut output);
            // Write the structure
            encoder.push(Header::Map(Some(1))).unwrap();

            encoder.text("contours", None).unwrap();
            encoder.push(Header::Array(Some(contours.len()))).unwrap();
            for contour in contours {
                encoder.push(Header::Array(Some(contour.len()))).unwrap();
                for contour_line in contour {
                    encoder
                        .push(Header::Array(Some(contour_line.len())))
                        .unwrap();
                    for vertex in contour_line {
                        encoder.push(Header::Array(Some(2))).unwrap();
                        encoder.push(Header::Float(vertex.x)).unwrap();
                        encoder.push(Header::Float(vertex.y)).unwrap();
                    }
                }
            }

            encoder.flush().unwrap();
            Ok(output)
        }
        _ => Err(String::from("Expected an array of inputs")),
    }
}

#[wasm_func]
pub fn histogram(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = Decoder::from(input);

    match decoder.pull().unwrap() {
        Header::Array(Some(len)) => {
            if len != 2 {
                return Err(String::from("Expected array of 2 elements, got more"));
            }

            let values = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len)?,
                _ => return Err(String::from("Bad input")),
            };


            let edges = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len)?,
                Header::Positive(num_bins) => {
                    let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                    let step = (max - min) / (num_bins as f64);
                    (0..num_bins + 1).map(|x| min + (x as f64) * step).collect()
                }
                _ => return Err(String::from("Bad input")),
            };

            let counts = komet::histogram(&values, &edges);

            let mut output = Vec::<u8>::new();
            let mut encoder = Encoder::from(&mut output);
            // Write the structure
            encoder.push(Header::Map(Some(2))).unwrap();

            encoder.text("counts", None).unwrap();
            encoder.push(Header::Array(Some(counts.len()))).unwrap();
            for count in counts {
                encoder.push(Header::Positive(count)).unwrap();
            }

            encoder.text("edges", None).unwrap();
            encoder.push(Header::Array(Some(edges.len()))).unwrap();
            for edge in edges {
                encoder.push(Header::Float(edge)).unwrap();
            }
            encoder.flush().unwrap();
            Ok(output)
        }
        _ => Err(String::from("Expected an array of inputs")),
    }
}

#[wasm_func]
pub fn boxplot(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = Decoder::from(input);

    match decoder.pull().unwrap() {
        Header::Array(Some(len)) => {
            if len != 2 {
                return Err(String::from("Expected array of 2 elements, got more"));
            }

            let values = match decoder.pull().unwrap() {
                Header::Array(Some(len)) => read::read_float_array(&mut decoder, len)?,
                _ => return Err(String::from("Bad input")),
            };

            let whisker_pos = match decoder.pull().unwrap() {
                Header::Float(whisker_pos) => whisker_pos,
                _ => return Err(String::from("Bad input")),
            };

            let boxplot_stats = komet::boxplot(&values, whisker_pos);

            let mut output = Vec::<u8>::new();
            let mut encoder = Encoder::from(&mut output);
            // Write the structure
            encoder.push(Header::Map(Some(9))).unwrap();

            encoder.text("mean", None).unwrap();
            encoder.push(Header::Float(boxplot_stats.mean)).unwrap();

            encoder.text("median", None).unwrap();
            encoder.push(Header::Float(boxplot_stats.median)).unwrap();

            encoder.text("q1", None).unwrap();
            encoder.push(Header::Float(boxplot_stats.q1)).unwrap();

            encoder.text("q3", None).unwrap();
            encoder.push(Header::Float(boxplot_stats.q3)).unwrap();

            encoder.text("min", None).unwrap();
            encoder.push(Header::Float(boxplot_stats.min)).unwrap();

            encoder.text("max", None).unwrap();
            encoder.push(Header::Float(boxplot_stats.max)).unwrap();

            encoder.text("whisker-low", None).unwrap();
            encoder
                .push(Header::Float(boxplot_stats.whisker_low))
                .unwrap();

            encoder.text("whisker-high", None).unwrap();
            encoder
                .push(Header::Float(boxplot_stats.whisker_high))
                .unwrap();

            encoder.text("outliers", None).unwrap();
            encoder
                .push(Header::Array(Some(boxplot_stats.outliers.len())))
                .unwrap();
            for outlier in boxplot_stats.outliers {
                encoder.push(Header::Float(outlier)).unwrap();
            }

            encoder.flush().unwrap();
            Ok(output)
        }
        _ => Err(String::from("Expected an array of inputs")),
    }
}

#[wasm_func]
pub fn boxplot_alt(input: &[u8]) -> Result<Vec<u8>, String> {
    let values: Vec<f64> = input
        .chunks_exact(8)
        .map(|bytes| f64::from_be_bytes(bytes.try_into().expect("msg")))
        .collect();

    let whisker_pos = values[0];

    let boxplot_stats = komet::boxplot(&values[1..], whisker_pos);

    let mut output_values: Vec<f64> = vec![
        boxplot_stats.mean,
        boxplot_stats.median,
        boxplot_stats.min,
        boxplot_stats.max,
        boxplot_stats.q1,
        boxplot_stats.q3,
        boxplot_stats.whisker_low,
        boxplot_stats.whisker_high,
    ];
    output_values.extend(boxplot_stats.outliers);
    let p: Vec<[u8; 8]> = output_values
        .iter()
        .map(|value| f64::to_be_bytes(*value))
        .collect();

    Ok(p[..].concat())
}

#[wasm_func]
pub fn histogram_alt(bins: &[u8], values: &[u8]) -> Result<Vec<u8>, String> {
    let values: Vec<f64> = values
        .chunks_exact(8)
        .map(|bytes| f64::from_be_bytes(bytes.try_into().expect("msg")))
        .collect();
    let bins: Vec<i64> = bins
        .chunks_exact(8)
        .map(|bytes| i64::from_be_bytes(bytes.try_into().expect("msg")))
        .collect();

    let num_bins = bins[0];

    let edges: Vec<f64> = {
        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let step = (max - min) / (num_bins as f64);
        (0..num_bins + 1).map(|x| min + (x as f64) * step).collect()
    };

    let counts = komet::histogram(&values, &edges);

    let counts_binary: Vec<[u8; 8]> = counts
        .iter()
        .map(|value| u64::to_be_bytes(*value))
        .collect();
    // let edges_binary: Vec<[u8; 8]> = counts
    //     .iter()
    //     .map(|value| f64::to_be_bytes(*value))
    //     .collect();

    Ok(counts_binary[..].concat())
}

fn fft_impl(input: &[u8], direction: FftDirection) -> Result<Vec<u8>, String> {
    let mut decoder = Decoder::from(input);

    let mut values = match decoder.pull().unwrap() {
        Header::Array(Some(len)) => read::read_complex_array(&mut decoder, len).unwrap(),
        _ => return Err(String::from("Expected an array of inputs")),
    };

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft(values.len(), direction);
    fft.process(&mut values);
    if direction == FftDirection::Inverse {
        let normalization = 1. / (values.len() as f64);
        for value in values.iter_mut() {
            *value *= normalization;
        }
    }

    let mut output = Vec::<u8>::new();
    let mut encoder = Encoder::from(&mut output);

    encoder.push(Header::Array(Some(values.len()))).unwrap();
    for value in values {
        encoder.push(Header::Array(Some(2))).unwrap();
        encoder.push(Header::Float(value.re)).unwrap();
        encoder.push(Header::Float(value.im)).unwrap();
    }

    Ok(output)
}

#[wasm_func]
fn fft(input: &[u8]) -> Result<Vec<u8>, String> {
    fft_impl(input, FftDirection::Forward)
}

#[wasm_func]
fn ifft(input: &[u8]) -> Result<Vec<u8>, String> {
    fft_impl(input, FftDirection::Inverse)
}

#[wasm_func]
pub fn thomas_algorithm(a: &[u8], b: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder_a = Decoder::from(a);
    let mut decoder_b = Decoder::from(b);

    let a = match decoder_a.pull().unwrap() {
        Header::Array(Some(len)) => read::read_float_array_2d(&mut decoder_a, len)?,
        _ => return Err("input is not an array".to_string()),
    };
    let a = a.iter().map(|r| r.as_slice()).collect::<Vec<&[f64]>>();

    let b = match decoder_b.pull().unwrap() {
        Header::Array(Some(len)) => read::read_float_array(&mut decoder_b, len)?,
        _ => return Err("input is not an array".to_string()),
    };

    let x = komet::thomas_algorithm(&a, &b);

    let mut output = Vec::new();
    let mut encoder = Encoder::from(&mut output);

    encoder.push(Header::Array(Some(x.len()))).unwrap();

    for value in x {
        encoder.push(Header::Float(value)).unwrap();
    }

    encoder.flush().unwrap();

    Ok(output)
}
