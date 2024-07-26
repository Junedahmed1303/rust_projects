use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let file_path = "numbers.txt"; // Update file path as needed

    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file '{}'", file_path))?;
    
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    
    for line_result in reader.lines() {
        let line = line_result
            .with_context(|| format!("Failed to read line from file"))?;
        
        let number: i32 = line
            .trim()
            .parse()
            .with_context(|| format!("Failed to parse number in line: '{}'", line))?;
        
        sum += number;
    }

    println!("Sum of numbers in '{}' is: {}", file_path, sum);

    Ok(())
}