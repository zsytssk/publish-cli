use crate::{tools::format_time, venus::ImageInfo};

pub fn format_images_info(images: &Vec<ImageInfo>) -> Vec<String> {
    images
        .iter()
        .map(|item| {
            let (_, image_name) = item.image_address.rsplit_once("/").unwrap();
            let time = format_time(&item.create_time);
            format!("[{}:{}] {}", time, item.creator, image_name)
        })
        .collect()
}
