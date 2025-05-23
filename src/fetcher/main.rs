// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    //use asimov_apify_module::api::*;
    use asimov_module::getenv;
    use clientele::SysexitsError::*;
    use std::io::stdout;

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
        return Ok(EX_OK);
    }

    // Obtain the Apify API token from the environment:
    let Some(_api_key) = getenv::var_secret("APIFY_TOKEN") else {
        return Ok(EX_CONFIG); // not configured
    };

    // Process each of the given URL arguments:
    for _url in urls {
        let response = "{}"; // TODO

        // Serialize the response data:
        if cfg!(feature = "pretty") {
            let response_json: serde_json::Value = serde_json::from_str(&response)?;
            colored_json::write_colored_json(&response_json, &mut stdout())?;
            println!();
        } else {
            println!("{}", response);
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-apify-fetcher requires the 'std' feature")
}
