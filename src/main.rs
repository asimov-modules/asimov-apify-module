// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    //use asimov_apify_module::api::*;
    use clientele::SysexitsError;
    use secrecy::SecretString;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Configure logging:
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    // Parse URLs from command-line arguments:
    let urls: Vec<String> = args
        .iter()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into())
        .collect();
    if urls.is_empty() {
        return Ok(SysexitsError::EX_OK);
    }

    // Obtain the Apify API token from the environment:
    let _api_key = SecretString::from(std::env::var("APIFY_TOKEN")?);

    // Send the request and block while waiting for the response:
    // TODO

    Ok(SysexitsError::EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-apify-importer requires the 'std' feature")
}
