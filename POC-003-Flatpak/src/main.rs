use ashpd::desktop::screenshot::Screenshot;

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

    Ok(())
}