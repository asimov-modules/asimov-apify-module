// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_apify_module::{api::*, find_actor_for, jq};
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

    let Ok(manifest) = asimov_module::ModuleManifest::read_manifest("apify")
        .inspect_err(|e| eprintln!("failed to read module manifest: {e}"))
    else {
        return Ok(EX_CONFIG);
    };

    // Obtain the Apify API token from the environment:
    let Ok(api_key) = manifest
        .variable("token", None)
        .inspect_err(|e| eprintln!("failed to get token: {e}"))
    else {
        return Ok(EX_CONFIG); // not configured
    };

    let api = Apify::new(api_key.into())?;

    // Process each of the given URL arguments:
    for url in urls {
        // Find the appropriate actor based on the URL prefix:
        let Some(actor) = find_actor_for(&url) else {
            return Ok(EX_UNAVAILABLE); // not supported
        };

        // Send the request and block while waiting for the response:
        let response = match actor.id {
            "apify~google-search-scraper" => jq::google_search()
                .filter_json_str(api.google_search(&url.parse().map_err(|_| EX_DATAERR)?)?)?,
            "C2Wk3I6xAqC4Xi63f" => jq::x_follows()
                .filter_json_str(api.x_follows(&url.parse().map_err(|_| EX_DATAERR)?)?)?,
            "VhxlqQXRwhW8H5hNV" => jq::linkedin_profile()
                .filter_json_str(api.linkedin_profile(&url.parse().map_err(|_| EX_DATAERR)?)?)?,
            "dSCLg0C3YEZ83HzYX" => jq::instagram_profile()
                .filter_json_str(api.instagram_profile(&url.parse().map_err(|_| EX_DATAERR)?)?)?,
            _ => {
                return Ok(EX_UNAVAILABLE); // not supported
            }
        };

        // Serialize the response data:
        if cfg!(feature = "pretty") {
            colored_json::write_colored_json(&response, &mut stdout())?;
            println!();
        } else {
            println!("{}", serde_json::to_string(&response).unwrap());
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-apify-importer requires the 'std' feature")
}
