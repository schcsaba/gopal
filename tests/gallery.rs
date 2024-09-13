use gopal::pages::gallery::{get_images, Image};
use std::fs;
use tempfile::TempDir;
use tokio;

#[test]
fn test_image_partial_eq() {
    let image1 = Image {
        id: 1,
        filename: "test.webp".to_string(),
        aspect_ratio: 1.5,
    };
    let image2 = Image {
        id: 1,
        filename: "test.webp".to_string(),
        aspect_ratio: 1.5,
    };
    let image3 = Image {
        id: 2,
        filename: "test.webp".to_string(),
        aspect_ratio: 1.5,
    };
    let image4 = Image {
        id: 1,
        filename: "other.webp".to_string(),
        aspect_ratio: 1.5,
    };
    let image5 = Image {
        id: 1,
        filename: "test.webp".to_string(),
        aspect_ratio: 1.6,
    };

    assert_eq!(image1, image2);
    assert_ne!(image1, image3);
    assert_ne!(image1, image4);
    assert_ne!(image1, image5);
}

#[tokio::test]
async fn test_get_images() {
    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();
    let gallery_path = temp_dir.path().to_str().unwrap().to_string();

    // Create some test images
    fs::write(temp_dir.path().join("image1.webp"), "dummy content").unwrap();
    fs::write(temp_dir.path().join("image2.webp"), "dummy content").unwrap();
    fs::write(temp_dir.path().join("not_an_image.txt"), "dummy content").unwrap();

    // Set the environment variable
    std::env::set_var("GALLERY_PATH", &gallery_path);

    // Call the function
    let mut result = get_images().await.unwrap();

    // Sort the results by filename to ensure consistent ordering
    result.sort_by(|a, b| a.filename.cmp(&b.filename));

    // Assert the results
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].filename, "image1.webp");
    assert_eq!(result[1].filename, "image2.webp");
    assert!(result[0].id < 2 && result[1].id < 2);
    assert_eq!(result[0].aspect_ratio, 1.5);
    assert_eq!(result[1].aspect_ratio, 1.5);
}

#[tokio::test]
async fn test_get_images_empty_directory() {
    // Create an empty temporary directory
    let temp_dir = TempDir::new().unwrap();
    let gallery_path = temp_dir.path().to_str().unwrap().to_string();

    // Set the environment variable
    std::env::set_var("GALLERY_PATH", &gallery_path);

    // Call the function
    let result = get_images().await.unwrap();

    // Assert the results
    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn test_get_images_nonexistent_directory() {
    // Set a non-existent directory path
    std::env::set_var("GALLERY_PATH", "/path/that/does/not/exist");

    // Call the function
    let result = get_images().await.unwrap();

    // Assert the results
    assert_eq!(result.len(), 0);
}
