use axum::{http::StatusCode, response::Response};
use std::{
    fs::{copy, write},
    path::PathBuf,
};
use webp::Encoder;

use super::http::{response_error, response_file};

pub fn save_image_to_webp(image: &image::DynamicImage, path: &PathBuf) -> Result<(), String> {
    let encoder = match Encoder::from_image(image) {
        Ok(e) => e,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let webp_memory = encoder.encode(100f32);

    match write(path, &*webp_memory) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn save_resized_image(
    image: image::DynamicImage,
    width: Option<u32>,
    original_path: &PathBuf,
    target_path: &PathBuf,
) -> Response {
    if width.is_none() {
        return response_file(target_path).await;
    }

    if image.width() <= width.unwrap() {
        copy(original_path, target_path).ok();
        return response_file(target_path).await;
    }

    let resize_height = width.unwrap() * image.height() / image.width();
    let resized_image = image.thumbnail(width.unwrap(), resize_height);

    match resized_image.save(target_path.clone()) {
        Ok(_) => response_file(target_path).await,
        Err(_) => response_error(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
