use image;
//use image_compare::Algorithm;
use image_compare::Metric;

fn matcher(img1: &str, img2: &str) {
    let image_one = image::open(img1)
        .expect("Could not find test-image")
        .into_luma8();
    let image_two = image::open(img2)
        .expect("Could not find test-image")
        .into_luma8();
    let mut result =
        image_compare::gray_similarity_histogram(Metric::Hellinger, &image_one, &image_two)
            .expect("Images had different dimensions");
    result = 1.0 - result;
    println!("{:?}", result);
}

fn main() {
    matcher("circ1.jpg", "circ2.jpg");
    /*
    let image_one = image::open("ads1.jpg")
        .expect("Could not find test-image")
        .into_rgb8();
    let image_two = image::open("ads2.jpg")
        .expect("Could not find test-image")
        .into_rgb8();
    let result = image_compare::rgb_hybrid_compare(&image_one, &image_two)
        .expect("Images had different dimensions");

    println!("{:?}", result.score);
    // 1 and 2 => 85.56%
    // 1 and 3 => 81.71%
    // 1 and 4 => 79.2%
    // */

    /*
    let image_one = image::open("ads1.jpg")
        .expect("Could not find test-image")
        .into_luma8();
    let image_two = image::open("ads4.jpg")
        .expect("Could not find test-image")
        .into_luma8();
    let result =
        image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, &image_one, &image_two)
            .expect("Images had different dimensions");

    println!("{:?}", result.score);
    // 1 and 2: 87.6
    // 1 and 3: 83.31
    // 1 and 4: 82.38
    // */
    /*
        let image_one = image::open("ads1.jpg")
            .expect("Could not find test-image")
            .into_luma8();
        let image_two = image::open("ads3.jpg")
            .expect("Could not find test-image")
            .into_luma8();
        let result =
            image_compare::gray_similarity_histogram(Metric::Hellinger, &image_one, &image_two)
                .expect("Images had different dimensions");
        println!("{:?}", result);
    */
    // 1 and 2 give 11.75
    // 1 and 3 give 24.03
    // 1 and 4 give 33.09
}
