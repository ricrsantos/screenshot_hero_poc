use ashpd::desktop::screenshot::Screenshot;
// use image::GenericImageView;
use std::fs;
use url::Url;

#[tokio::main]
async fn main() -> ashpd::Result<()> {
    println!("Solicitando screenshot...");

    let response = Screenshot::request()
        .interactive(true)
        .modal(true)
        .send()
        .await?
        .response()?;

    println!("URI: {}", response.uri());

    let path = Url::parse(response.uri().as_str())
        .unwrap()
        .to_file_path()
        .unwrap();

    println!("Path: {}", path.display());

    let metadata = fs::metadata(&path).unwrap();

    println!("Tamanho arquivo: {} bytes", metadata.len());

    let img = image::open(&path).unwrap();

    println!("Largura: {}", img.width());
    println!("Altura: {}", img.height());

    Ok(())
}
