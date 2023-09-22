use image::GenericImage;
use image::GenericImageView;

fn stitch(images: &Vec<image::DynamicImage>, gutters: u32) -> image::DynamicImage {
    // Get the dimensions of the first image
    let (width, height) = images[0].dimensions();

    // Create a new image buffer with the height of the first image and the width of all images
    let mut stitched = image::DynamicImage::new_rgb8(width * images.len() as u32 + (images.len() - 1 ) as u32 * gutters, height);
    
    // Set all pixels to white using a mutable view
    let mut view = stitched.as_mut_rgb8().unwrap();

    // Iterate over all images
    for (i, img) in images.iter().enumerate() {
        // copy the image into the stitched image using views
        stitched.copy_from(img, (width + gutters) * i as u32, 0).unwrap();
    }

    // Return the stitched image
    stitched
}

fn main() {
    // Parse commmand line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if there are enough arguments
    if args.len() < 2 {
        println!("Usage: {} <filename> ...", args[0]);
        std::process::exit(1);
    }

    // Load the images from the files
    let mut images: Vec<image::DynamicImage> = Vec::new();
    for i in 1..args.len() {
        let img = image::open(&args[i]).unwrap();
        images.push(img);
    }

    // Print the number of images
    println!("Loaded {} images", images.len());

    // Stitch the images together
    let stitched = stitch(&images, 64);

    // Save the stitched image
    stitched.save("stitched.png").unwrap();

    println!("Stitched image saved to stitched.png");
}