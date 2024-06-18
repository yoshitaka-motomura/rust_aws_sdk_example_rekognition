use anyhow::{Context, Ok};
use clap::{arg, command, Command};
use aws_sdk_example::handlers::detect::get_detect_labels;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = command!() // requires `cargo` feature
        .name("rekognition")
        .version("0.1.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("detect")
                .arg(arg!(<path> "Required Detecting Image Path"))
                .about("Detect labels from image elements"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("detect", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("required");
            get_detect_labels(path.to_string()).await.context("Failed")?;
        }
        _ => unreachable!(""),
    }

    Ok(())
}
