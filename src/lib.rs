use pyo3::prelude::*;
use image::io::Reader as ImageReader;


pub type Root = Vec<Vec<[bool; 2]>>;

#[pyfunction]
fn process_image(image: String, json: String, out_path: Option<String>) -> PyResult<()> {
    let value = serde_json::from_str::<Root>(&json).unwrap();
    let mut img = ImageReader::open(format!("./{}", image))
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8();

    let mut img_iter = img.pixels_mut();

    for row in value.into_iter() {
        for (col, pix) in row.into_iter().zip(&mut img_iter) {
            if col != [false, false] {
                *pix = image::Rgba([0, 0, 0, 0]);
            }
        }
    }

    img.save(out_path.unwrap_or(String::from("out.png")))
        .unwrap();
    Ok(())
}

#[pymodule]
fn mask_example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_image, m)?)?;
    Ok(())
}