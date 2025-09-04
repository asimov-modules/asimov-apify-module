// This is free and unencumbered software released into the public domain.

use clap::Parser;
use clientele::StandardOptions;

/// asimov-apify-importer
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The output format.
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    urls: Vec<String>,
}

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_apify_module::{api::*, find_actor_for, jq};
    use clientele::SysexitsError::*;
    use std::io::stdout;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(&args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    if options.urls.is_empty() {
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
    for url in options.urls {
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
