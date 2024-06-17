
use aws_sdk_rekognition as rekognition;
use dotenv::dotenv;
use env_logger::Env;
use aws_smithy_types::Blob;
use anyhow::{Result, Context};
use std::env;
use tabled::{Tabled, Table};
use tabled::settings::{Modify, object::Segment};
use color_print::cprintln;
use terminal_size::{Width, terminal_size};

#[derive(Tabled)]
struct LabelCollection {
    name: String,
    confidence: f32,
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }
    let config = aws_config::load_from_env().await;
    let client = rekognition::Client::new(&config);

    let image = std::fs::read(&args[1]).context("Failed to read image file")?;
    let input = rekognition::types::Image::builder().bytes(Blob::new(image.to_vec())).build();
 

    let result = client.detect_labels()
        .image(input)
        .max_labels(10)
        .send()
        .await.context("Failed to detect labels")?;

    //let mut tags:Vec<String> = Vec::new();
    let mut labels: Vec<LabelCollection> = vec![];

    result.labels().iter().for_each(|label| {
        let name = label.name().unwrap_or("Unknown");
        let confidence = label.confidence().unwrap_or( 0.0);
        //tags.push(name.to_string());
        let item = LabelCollection {
            name: name.to_string(), 
            confidence: confidence,
        };
        labels.push(item);
    });

    cprintln!("<green>Source of Detected Labels:</green>\n{:?}", &args[1]);
    let mut table = Table::new(&labels);
    if let Some((Width(width), _)) = terminal_size() {
        table.with(Modify::new(Segment::all()).with(tabled::settings::Width::wrap(width as usize - 4))); // 4 is for padding/margin
    }
    println!("{}", table.to_string());

    Ok(())
}