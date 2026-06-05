use ashpd::desktop::screenshot::Screenshot;
use std::fs;
use url::Url;

#[tokio::main]
async fn main() -> ashpd::Result<()> {
    println!("Requesting screenshot...");

    let response = Screenshot::request()
        .interactive(true)
        .modal(true)
        .send()
        .await?
        .response()?;

    let uri = response.uri();
    println!("URI: {uri}");

    let path = match Url::parse(uri.as_str()) {
        Ok(parsed_url) => match parsed_url.to_file_path() {
            Ok(file_path) => {
                println!("Converted filesystem path: {}", file_path.display());
                Some(file_path)
            }
            Err(_) => {
                eprintln!("Warning: could not convert URI to a filesystem path.");
                println!("Converted filesystem path: <not available>");
                None
            }
        },
        Err(error) => {
            eprintln!("Warning: invalid URI returned by portal: {error}");
            println!("Converted filesystem path: <invalid URI>");
            None
        }
    };

    let Some(path) = path else {
        println!("File exists: false");
        return Ok(());
    };

    let file_exists = path.exists();
    println!("File exists: {file_exists}");

    if !file_exists {
        eprintln!(
            "Warning: screenshot path is not accessible from current sandbox: {}",
            path.display()
        );
        return Ok(());
    }

    match fs::metadata(&path) {
        Ok(metadata) => println!("File size (bytes): {}", metadata.len()),
        Err(error) => {
            eprintln!("Warning: failed to read file metadata: {error}");
            return Ok(());
        }
    }

    match image::open(&path) {
        Ok(img) => {
            println!("Image width: {}", img.width());
            println!("Image height: {}", img.height());
        }
        Err(error) => {
            eprintln!("Warning: failed to open image file: {error}");
        }
    }

    Ok(())
}
