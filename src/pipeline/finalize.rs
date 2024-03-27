use std::path::PathBuf;

use libvips::{ops, VipsImage};
use libvips::ops::{JpegsaveOptions, WebpsaveOptions};
use log::error;

use crate::cache;
use crate::parameters::{Quality, UrlParameters};
use crate::pipeline::{PipelineError, PipelineResult};
use crate::services::formats::OutputFormat;
use crate::services::vips::get_error_message;

pub(crate) async fn run(image: &VipsImage, url_parameters: &UrlParameters<'_>, output_format: &OutputFormat) -> PipelineResult<PathBuf> {
    match output_format {
        // OutputFormat::Avif => finalize_avif(image, url_parameters),
        OutputFormat::Webp => finalize_webp(image, url_parameters),
        _ => finalize_jpg(image, url_parameters)
    }
}

// Very slow for now
// fn finalize_avif(image: &VipsImage, url_parameters: &UrlParameters<'_>) -> PipelineResult<PathBuf> {
//
//     let cache_path = cache::get_path_from_url_parameters(url_parameters, &OutputFormat::Avif);
//
//     if ops::heifsave_with_opts(image, &cache_path, &HeifsaveOptions {
//         q: match url_parameters.quality {
//             Quality::Custom(quality) => quality as i32,
//             Quality::Default => 50,
//         },
//         compression: ForeignHeifCompression::Av1,
//         ..HeifsaveOptions::default()
//     }).is_err() {
//         error!("Failed to save AVIF image {}: {}", url_parameters.path.to_string_lossy(), get_error_message());
//         return Err(PipelineError("Failed to save image".to_string()));
//     }
//
//     Ok(cache_path.into())
//
// }

fn finalize_webp(image: &VipsImage, url_parameters: &UrlParameters<'_>) -> PipelineResult<PathBuf> {

    let cache_path = cache::get_path_from_url_parameters(url_parameters, &OutputFormat::Webp);

    if ops::webpsave_with_opts(image, &cache_path, &WebpsaveOptions {
        q: match url_parameters.quality {
            Quality::Custom(quality) => quality as i32,
            Quality::Default => 70,
        },
        ..WebpsaveOptions::default()
    }).is_err() {
        error!("Failed to save WEBP image {}: {}", url_parameters.path.to_string_lossy(), get_error_message());
        return Err(PipelineError("Failed to save image".to_string()));
    }

    Ok(cache_path.into())

}

fn finalize_jpg(image: &VipsImage, url_parameters: &UrlParameters<'_>) -> PipelineResult<PathBuf> {

    let cache_path = cache::get_path_from_url_parameters(url_parameters, &OutputFormat::Webp);

    if ops::jpegsave_with_opts(image, &cache_path, &JpegsaveOptions {
        q: match url_parameters.quality {
            Quality::Custom(quality) => quality as i32,
            Quality::Default => 79,
        },
        ..JpegsaveOptions::default()
    }).is_err() {
        error!("Failed to save JPG image {}: {}", url_parameters.path.to_string_lossy(), get_error_message());
        return Err(PipelineError("Failed to save image".to_string()));
    }

    Ok(cache_path.into())

}