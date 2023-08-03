use itertools::Itertools;
use poloto::build;
use splines::Key;
use std::{
    collections::HashMap,
    fs::File,
    hash::BuildHasher,
    io::{self, BufReader},
};
mod tmp;
fn load_hdi() -> io::Result<HashMap<String, Vec<f64>>> {
    let file = File::open("./data/HDI.csv")?;
    let buffer = BufReader::new(file);
    let mut csv = csv::Reader::from_reader(buffer);
    Ok(csv
        .records()
        .flatten()
        .filter_map(|row| {
            row.get(1).and_then(|name| {
                (5..37)
                    .map(|i| row.get(i).and_then(|s| s.parse::<f64>().ok()))
                    .collect::<Option<Vec<_>>>()
                    .map(|data| (name.to_string(), data))
            })
        })
        .collect::<HashMap<String, Vec<f64>>>())
}

fn load_gdp() -> io::Result<HashMap<String, Vec<f64>>> {
    let file = File::open("./data/GDP.csv")?;
    let buffer = BufReader::new(file);
    let mut csv = csv::Reader::from_reader(buffer);
    Ok(csv
        .records()
        .flatten()
        .filter_map(|row| {
            row.get(0).and_then(|name| {
                (1..33)
                    .map(|i| row.get(i).and_then(|s| s.parse::<f64>().ok()))
                    .collect::<Option<Vec<_>>>()
                    .map(|data| (name.to_string(), data))
            })
        })
        .collect::<HashMap<String, Vec<f64>>>())
}

fn load_gini() -> Option<HashMap<String, Vec<f64>>> {
    let file = File::open("./data/Gini.csv").ok()?;
    let buffer = BufReader::new(file);
    let mut csv = csv::Reader::from_reader(buffer);
    Some(
        csv.records()
            .flatten()
            .filter_map(|row| {
                row.get(0).and_then(|name| {
                    let all = (1..33)
                        .map(|i| row.get(i).and_then(|s| s.parse::<f64>().ok()))
                        .enumerate()
                        .collect_vec();

                    let keys = all
                        .iter()
                        .filter_map(|(y, v)| v.map(|value| (value, (y + START_YEAR) as f64)))
                        .map(|(v, y)| Key::new(y, v, splines::Interpolation::Cosine))
                        .collect_vec();

                    if keys.is_empty() {
                        None
                    } else {
                        // let first = keys[0];
                        // let last = keys[keys.len() - 1];
                        // let spline =
                        //     splines::Spline::from_vec([vec![first], keys, vec![last]].concat());
                        let spline = splines::Spline::from_vec(keys);

                        Some((name.to_string(), sample_spline(all, spline)))
                    }
                })
            })
            .collect::<HashMap<String, Vec<f64>>>(),
    )
}

fn sample_spline(all: Vec<(usize, Option<f64>)>, spline: splines::Spline<f64, f64>) -> Vec<f64> {
    all.iter()
        .map(|(year, value)| {
            value.map_or_else(
                || spline.clamped_sample((*year + START_YEAR) as f64).unwrap(),
                |value| value,
            )
        })
        .collect_vec()
}

const START_YEAR: usize = 1990;
const END_YEAR: usize = 2021;

fn value_from_year(year: usize, data: &[f64]) -> Option<f64> {
    year.checked_sub(START_YEAR)
        .and_then(|index| data.get(index).copied())
}

fn main() {
    let mut data = vec![];

    if let (Ok(hdi), Ok(gdp)) = (load_hdi(), load_gdp()) {
        for (name, hdi_points) in hdi.iter() {
            if let Some(gdp_points) = gdp.get(name) {
                for point in gdp_points.iter().zip(hdi_points.iter()) {
                    data.push(point);
                }
            }
        }

        let plots = poloto::plots!(poloto::build::plot("").scatter(data));

        poloto::frame_build()
            .data(poloto::plots!(
                build::markers(vec![2000., 2030.], [0., 1.]),
                plots
            ))
            .build_and_label(("HDI vs GDP", "HDI", "GDP"))
            .append_to(poloto::header().light_theme())
            .render_string();
    }
}

// if let Some(Some(data)) = load_gini().map(|m| m.get("United States").cloned()) {
//     let line = data
//         .iter()
//         .enumerate()
//         .map(|(i, value)| ((i + START_YEAR) as f64, *value));

//     let plots = poloto::plots!(poloto::build::plot("Costa Rica").line(line));

//     poloto::frame_build()
//         .data(poloto::plots!(
//             build::markers(vec![2000., 2030.], [20., 60.]),
//             plots
//         ))
//         .build_and_label(("gaussian", "x", "y"))
//         .append_to(poloto::header().dark_theme())
//         .render_stdout();
// }
//
//
