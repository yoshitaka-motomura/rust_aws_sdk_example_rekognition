use aws_sdk_rekognition::types::builders::ImageBuilder;
use aws_sdk_rekognition::types::S3Object;
use aws_sdk_rekognition as rekognition;
use aws_sdk_rekognition::operation::detect_labels::DetectLabelsOutput;
use serde::{Serialize, Deserialize};
use serde_json;
use dotenv::dotenv;
use std::env;
use env_logger::Env;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
struct LabelData {
    name: String,
    confidence: f32,
}

#[tokio::main]
async fn main() -> Result<(), rekognition::Error> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load the AWS SDK credentials from the environment
    let config = aws_config::load_from_env().await;
    // Create a Rekognition client
    let client = aws_sdk_rekognition::Client::new(&config);

    // Get the bucket name and object key from environment variables
    let bucket_name = env::var("AWS_S3_BUCKET_NAME").expect("BUCKET_NAME must be set");
    info!("Bucket Name: {}", bucket_name);
    let object_key = env::var("AWS_S3_OBJECT_KEY").expect("OBJECT_KEY must be set");
    info!("Object Key: {}", object_key);

   // Create an S3Object with the bucket name and object key
    let s3_image = ImageBuilder::default().s3_object({
        S3Object::builder()
            .bucket(bucket_name.to_string())
            .name(object_key.to_string())
            .build()
    }).build();

    // Call the DetectLabels operation
    let result: DetectLabelsOutput = match client.detect_labels().image(s3_image)
    .max_labels(10) // Maximum number of labels to return
    .min_confidence(90.0) // Minimum confidence level for labels to be returned
    .send().await {
        Ok(output) => output,
        Err(error) => {
            println!("No Labels: {:?}", error);
            return Ok(());
        }  
    };

    // Iterate over the labels and print them
    let mut labels: Vec<LabelData> = Vec::new();
    result.labels().iter().for_each(|label| {
        labels.push(LabelData {
            name: label.name().unwrap().to_string(),
            confidence: label.confidence().unwrap(),
        });
    });

    // Serialize the labels to JSON
    let json = serde_json::to_string_pretty(&labels).unwrap();

    println!("⚡ Rekognition Result.\n{}", json);
    
    Ok(())
}
