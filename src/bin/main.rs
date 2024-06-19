use anyhow::{Context, Ok};
use clap::{arg, command, Command};
use aws_sdk_example::{handlers::detect::get_detect_labels, utils::{clean_cache_dir, retrive_file_from_cache}};
use url::Url;
use color_print::{cprintln, cformat};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let description = cformat!("<bold>Amazon Rekognition CLI</bold>");
    let matches = command!()
        .name("rekognition")
        .about(description.to_string())
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("detect")
                .arg(arg!(<path> "Required Detecting Image Path Or Image URL"))
                .about("Detect labels from image or image URL"),
        )
        .subcommand( 
            Command::new("clean")
                .about("Clean the cache directory"),
            )
        .get_matches();

    match matches.subcommand() {
        Some(("detect", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("required path").as_ref();

            let path_str = if Url::parse(path).is_ok() {
                cprintln!("<bold>The file is being downloaded because the specified image is a URL...</bold>");
                retrive_file_from_cache(path).await.context("Failed")?
             } else {
                path.to_string()
             };
            get_detect_labels(path_str).await.context("Failed")?;
        }
        Some(("clean", _)) => {
            cprintln!("<bold>Cleaning the cache directory...</bold>");
            clean_cache_dir().context("Failed")?;
        }
        _ => unreachable!("Woops!"),
    }

    Ok(())
}
