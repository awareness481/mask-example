use pyo3::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use image::{io::Reader as ImageReader, RgbaImage, ImageBuffer, GenericImageView};


pub type Root = Vec<Vec<Vec<bool>>>;

#[pyfunction]
fn process_image(image: String, json: String, out_path: Option<String>) -> PyResult<()> {
    let value = serde_json::from_str::<Root>(&json).unwrap();
    let img = ImageReader::open(format!("./{}", image)).unwrap().decode().unwrap();

    let (width, height) = img.dimensions();
    let mut out: RgbaImage = ImageBuffer::new(width, height);

    for (i, row) in value.iter().enumerate() {

        for (j, col) in row.iter().enumerate() {
            if col.into_iter().any(|x| *x) {
                *out.get_pixel_mut(j.try_into().unwrap(), i.try_into().unwrap()) = img.get_pixel(j.try_into().unwrap(), i.try_into().unwrap());
            } else {
                *out.get_pixel_mut(j.try_into().unwrap(), i.try_into().unwrap()) = image::Rgba([0, 0, 0, 0]);
            }
        }
    }

    out.save(out_path.unwrap_or(String::from("out.png"))).unwrap();
    Ok(())
}

#[pymodule]
fn mask_example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_image, m)?)?;
    Ok(())
}