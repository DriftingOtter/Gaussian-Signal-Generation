use std::io;
use rand::{self, Rng};
use regex::Regex;
use plotters::prelude::*;


const OUT_FILE_NAME: &str = "/home/daksh/Documents/random_noise_generator/src/histogram.png";


fn get_generation_resolution() -> u32 {
    let mut gen_resolution: String = String::new();

    println!("Enter Noise Generation Resolution");

    io::stdin()
        .read_line(&mut gen_resolution)
        .expect("Could not read Generation Resolution.");

    println!();

    let gen_resolution: u32 = gen_resolution.trim().parse().unwrap();

    return gen_resolution;
}

fn get_sample_width() -> f64 {
    let mut sample_width: String = String::new();

    println!("Enter Sample Width");

    io::stdin()
        .read_line(&mut sample_width)
        .expect("Could not read Generation Resolution.");

    println!();

    let sample_width: f64 = sample_width.trim().parse().unwrap();

    return sample_width;
}

fn get_sample_center() -> f64 {
    let mut sample_center: String = String::new();

    println!("Enter Sample Center");
    
    io::stdin()
        .read_line(&mut sample_center)
        .expect("Could not read Generation Resolution.");

    println!();

    let sample_center: f64 = sample_center.trim().parse().unwrap();

    return sample_center;
}

fn get_sample_size() -> u32 {
    let mut sample_size: String = String::new();

    println!("Enter N Size");

    io::stdin()
        .read_line(&mut sample_size)
        .expect("Could not read sample size.");

    println!();

    let sample_size: u32 = sample_size.trim().parse().unwrap();

    return sample_size;
}

fn get_binning_type() -> bool {
    let mut binning_type = String::new();

    println!("Would you like automatic binning? Or Manual? (A/M)");

    io::stdin()
        .read_line(&mut binning_type)
        .expect("Could not read binning type.");

    println!();

    let binning_type = binning_type.trim().to_lowercase();

    if binning_type == "m" {
        return true;
    } else {
        return false;
    }
}

fn get_sample(gen_count: u32, mean: f64, std_dev: f64) -> f64 {
    let mut smpl = 0.0;

    // Generate noise
    for _ in 0..gen_count - 1 {
        smpl += rand::thread_rng().gen_range(0.0..1.0);
    }

    // Normalize smpl to mean
    smpl = smpl - (gen_count as f64 / 2.0);

    // Apply characteristics
    smpl = (smpl * std_dev) + mean;

    return smpl;
}

fn set_buffer_contents(mut sample_buffer: Vec<f64>, smpl_size: u32, gen_res: u32, mean: f64, std_dev: f64) -> Vec<f64> {
    for _ in 0..smpl_size {
        sample_buffer.push(get_sample(gen_res, mean, std_dev));
    }

    return sample_buffer;
}

fn get_bin_size() -> u32 {
    let mut bin_size = String::new();

    println!("Enter Bin Count");

    io::stdin()
        .read_line(&mut bin_size)
        .expect("Could not read bin count.");

    println!();

    let bin_size: u32 = bin_size.trim().parse().unwrap();
    
    return bin_size;
}

fn get_range_bounds(range: String) -> (f64, f64) {
    let range_regex = r"^([0-9]*(?:\.[0-9]+)?)-([0-9]*(?:\.[0-9]+)?)$";
    let pattern = Regex::new(range_regex).unwrap();

    let mut lower_bound = String::new();
    let mut upper_bound = String::new();

    if let Some(captures) = pattern.captures(range.as_str()) {
        lower_bound = captures.get(1).map_or("", |m| m.as_str()).to_string();
        upper_bound = captures.get(2).map_or("", |m| m.as_str()).to_string();
    } else {
        println!("No match for input. Please enter range like 0.0-1.0.");
        return (0.0, 0.0); // or handle the error as needed
    }

    let lower_bound: f64 = lower_bound.trim().parse().unwrap();
    let upper_bound: f64 = upper_bound.trim().parse().unwrap();

    println!("Lower Bound: {} | Upper Bound: {}\n", lower_bound, upper_bound);

    return (lower_bound, upper_bound);
}

fn get_bin_range(bin_position: u32) -> String {
    let mut bin_range: String = String::new();

    println!("Enter #{} bin range (inclusive)", bin_position);

    io::stdin()
        .read_line(&mut bin_range)
        .expect("Could not read bin range.");

    println!();

    // Removing trailing characters
    let bin_range = bin_range.trim().to_string();

    return bin_range;
}

fn set_bin_contents(mut bin: Vec<u32>, bin_size: u32, sample_buffer: Vec<f64>) -> Vec<u32> {
    for bin_pos in 1..=bin_size {

        // Get range from regex
        let bin_range = get_bin_range(bin_pos);
        let (lower_bound, upper_bound) = get_range_bounds(bin_range);

        // Incrementing bin contents based on range
        for pos in 0..sample_buffer.len() {
            let sample = sample_buffer[pos];

            if sample >= lower_bound && sample <= upper_bound {
                bin[bin_pos as usize - 1] += 1;
            }
        }
    }
    return bin;
}

fn set_bin_contents_auto(mut bin: Vec<u32>, bin_size: u32, sample_buffer: Vec<f64>) -> Vec<u32> {

    let max_range = sample_buffer.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    // Ensure bin width is at least 1.0
    let bin_width = ((max_range / bin_size as f64).ceil()).max(1.0);

    for bin_pos in 0..bin_size {
        let lower_bound = (bin_pos as f64 * bin_width).ceil();
        let upper_bound = ((bin_pos + 1) as f64 * bin_width).ceil();

        // Incrementing bin contents based on range
        for &sample in &sample_buffer {
            if sample >= lower_bound && sample < upper_bound {
                bin[bin_pos as usize] += 1;
            }
        }
    }
    return bin;
}

fn set_histogram(domain_size: u32, data: Vec<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_bin_value = *data.iter().max().unwrap_or(&0);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(domain_size)
        .y_label_area_size(40)
        .margin(5)
        .caption("Noise Graph", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..domain_size).into_segmented(), 0u32..max_bin_value)?; 

    chart.configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Quantized Amplitude")
        .x_desc("Bins")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(data.iter().enumerate().map(|(idx, &value)| (idx as u32, value))),
    )?;

    root.present()?;
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn graph_samples(sample_buffer: Vec<f64>) {
    let binning_type = get_binning_type();

    if binning_type {

        // Create zero vector for storing samples
        let bin_size: u32 = get_bin_size();
        let bin: Vec<u32> = vec![0; bin_size as usize];

        // Set bin contents
        let bin = set_bin_contents(bin, bin_size, sample_buffer);

        println!("Bin: {:?}\n", bin);

        // Display graph
        set_histogram(bin_size, bin).expect("Could not display graph.");

    } else {

        // Create zero vector
        let bin_size: u32 = get_bin_size();
        let bin: Vec<u32> = vec![0; bin_size as usize];

        // Set bin contents
        let bin = set_bin_contents_auto(bin, bin_size, sample_buffer);

        // Display graph
        set_histogram(bin_size, bin).expect("Could not display graph.");
    }
}

fn main() {
    // Initialize buffer
    let sample_buffer: Vec<f64> = Vec::new();

    // Get noise characteristics 
    let gen_res   = get_generation_resolution();
    let mean      = get_sample_center();
    let std_dev   = get_sample_width();
    let smpl_size = get_sample_size();

    // Generate noise
    let sample_buffer = set_buffer_contents(sample_buffer, smpl_size, gen_res, mean, std_dev);

    println!("Sample Buffer: {:?}\n", sample_buffer);

    set_sample_to_bin(sample_buffer);

}

