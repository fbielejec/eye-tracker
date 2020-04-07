#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate opencv;
use opencv::core as cv;

use opencv::{
    core::{self, MatConstIterator, Point, Rect, Scalar, Size, Vec3, Vec2b, Vec3b, Vec3d, Vec3f, Vec4w},
    Error,
    prelude::*,
    Result,
    types::{VectorOfi32, VectorOfMat},
    highgui,
    imgcodecs
};

fn display_img(image: &Mat) -> opencv::Result<()> {
    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", image)?;
    highgui::wait_key(10000)?;

    Ok(())
}

/*
 * https://docs.opencv.org/master/d6/d6d/tutorial_mat_the_basic_image_container.html
 */
fn mat () -> opencv::Result<()> {

    // create a matrix
    // <opencv::core::Vec3<u8> as Trait>::typ
    let m = Mat::new_rows_cols_with_default(2, 2,
                                            Vec3b::typ(),
                                            // u8::typ(),
                                            Scalar::new(0., 0., 0., 255.))?;

    println!("{:#?}", m);

    // type: "CV_8UC1",
    println!("{:#?}", m.typ ());

    // get a row
    let row = m.at_row::<Vec3b>(0)?;
    println!("{:#?}", row);

    // get an element
    let row = m.at_2d::<Vec3b>(0, 0)?;
    println!("{:#?}", row);

    let points = m.at::<Vec3b>(0)?;
    println!("- POINTS - {:#?}", points);
    println!("- POINT - {:#?}", points [0]);

    // construct Mat using a literal
    let mat = Mat::from_slice_2d(&[
        &[ 1,  2,  3,  4],
        &[ 5,  6,  7,  8],
        &[ 9, 10, 11, 12],
        &[13, 14, 15, 16u8],
    ])?;

    // println!("DATA : {:#?}", mat.data ().unwrap ());


    Ok(())
}

/*
 * Demonstrates how to scan and reduce an image
 * https://docs.opencv.org/master/db/da5/tutorial_how_to_scan_images.html
 * https://github.com/opencv/opencv/blob/master/samples/cpp/tutorial_code/core/how_to_scan_images/how_to_scan_images.cpp
 */
fn scan_images () -> opencv::Result<()> {

    fn scan_image_and_reduce (m : &mut Mat, table : [u8; 256]) -> &Mat {
        if m.depth ().unwrap () != u8::depth() {
            panic!("Only char type matrices!");
        }

        let n_channels = m.channels ().unwrap ();
        let n_rows = m.rows ();
        let mut n_cols = m.cols ();
        // let mut channels = m.channels ().unwrap ();

        // continuous matrix can be accessed by col number:
        // let pixel = m.at_mut::<Vec3b>(262143).unwrap ();

        // or by row, col:
        // let pixel = m.at_2d::<Vec3b>(511, 511).unwrap ();
        // println!("rows: {} cols: {} elem-size: {}", n_rows, n_cols, channels);
        // println!("pixel [BGR]: {:#?}", pixel);
        // println!("pixel [B] {:#?}", pixel [0]);
        // println!("reduced pixel [B] {:#?}", table [pixel [0] as usize]);

        if m.is_continuous ().unwrap () {

            n_cols *= n_rows;
            // n_rows = 1;

            for i in 0..n_cols {
                let pixel = m.at_mut::<Vec3b>(i).unwrap ();
                for j in 0..n_channels as usize {
                    pixel [j] = table [pixel [j] as usize];
                }
            }

        }

        m
    }

    // println!("{} {}", imgcodecs::IMREAD_COLOR, imgcodecs::IMREAD_GRAYSCALE);

    let image : Mat = imgcodecs::imread("lena.jpg", imgcodecs::IMREAD_COLOR)?;

    // get an element
    // let row = image.at_2d::<Vec3b>(0, 0)?;
    // println!("{:#?}", row);

    // create a lookup table
    let divide_with: u8 = 100;
    let mut table: [u8; 256] = [0; 256];
    (0..256).for_each (|i: usize| {
        table [i] = divide_with * (i as u8 / divide_with);
    });

    println!("{} {}", table [0], table [255]);

    let mut image_clone: Mat = image.clone()?;
    let image_reduced = scan_image_and_reduce(&mut image_clone, table);


    // display_img(&image_reduced);

    // LUT
    // http://www.poumeyrol.fr/doc/opencv-rust/opencv/core/fn.lut.html
    let image_clone = image.clone()?;
    let look_up_table : Mat = Mat::from_slice (&table)?;

    // println!("{:#?}", look_up_table);
    // println!("{} {}", look_up_table.at_2d::<u8> (0,0)?, look_up_table.at_2d::<u8> (0,255)?);

    let mut image_reduced : Mat = Mat::new_rows_cols_with_default(image.rows (), image.cols (), Vec3b::typ(), Scalar::default())?;

    cv::lut(&image_clone, &look_up_table, &mut image_reduced);
    display_img(&image_reduced);

    // display_img(&image);

    Ok(())
}


// TODO : mask opearations
// https://docs.opencv.org/master/d7/d37/tutorial_mat_mask_operations.html
/*
 * Demonstrates how to apply a (sharpen) mask to an image
 */
fn image_mask () -> opencv::Result<()> {

    fn sharpen (image: &Mat, result: &mut Mat) -> opencv::Result<()> {

        let n_channels: i32 = image.channels ()? ;
        let n_rows: i32 = image.rows () ;
        let n_cols: i32 = image.cols () ;

        let mut tmp = 0;

        let n: i32 = n_rows * n_cols;
        for i in 3..(n - 3) {

            let previous_previous = image.at::<Vec3b>(i - 3)?;
            let previous = image.at::<Vec3b>(i - 1)?;
            let current  = image.at::<Vec3b>(i    )?;
            let next     = image.at::<Vec3b>(i + 1)?;
            let next_next     = image.at::<Vec3b>(i + 3)?;

            let pixel = result.at_mut::<Vec3b>(i)?;

            for j in 0..n_channels as usize {
                let (mut value, _) = 5u8.overflowing_mul(current[j]);
                let (mut value, _) = value.overflowing_sub(previous_previous[j]);
                let (mut value, _) = value.overflowing_sub(previous[j]);
                let (mut value, _) = value.overflowing_sub(next[j]);
                let (mut value, _) = value.overflowing_sub(next_next[j]);
                pixel [j] = value;
            }
        }

        Ok(())
    }

    let image : Mat = imgcodecs::imread("lena.jpg", imgcodecs::IMREAD_COLOR)?;
    let mut image_sharpened : Mat = Mat::new_rows_cols_with_default(image.rows (), image.cols (), image.typ ()?, Scalar::default())?;

    println!("{:#?}", image);

    sharpen (&image, &mut image_sharpened);

    display_img(&image_sharpened);

    Ok(())
}


fn main() {
    // let image = imgcodecs::imread("lena.jpg", imgcodecs::IMREAD_GRAYSCALE).unwrap();
    // display_img(&image).unwrap();

    // mat ().unwrap ();
    // scan_images ().unwrap ();
    image_mask ().unwrap ();

}
