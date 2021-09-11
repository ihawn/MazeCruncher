use image::RgbImage;
use ndarray::Array3;

//Convert maze matrix to Array3 and write that to an image
pub fn mtx_to_img(mtx: &[Vec<u8>], size: usize, name: String)
{
   
    println!("Writing Maze To File");
    let mut array: Array3<u8> = Array3::zeros((size, size, 3));
    
    for j in 0..size
    {
        for i in 0..size
        {

            //Determine which colors to assign based on the maze matrix
            if mtx[i][j] < u8::MAX
            {
                if mtx[i][j] == 0 { array = set_array_value(array, j, i, 255, 255, 255); } //space
                else if mtx[i][j] == 1 { array = set_array_value(array, j, i, 255, 0, 0); } //travelled once
                else
                {
                    if is_false_blue(mtx, i, j) { array = set_array_value(array, j, i, 255, 0, 0); } //travelled >1 but in the solution path
                    else { array = set_array_value(array, j, i, 0, 0, 255); }  //travelled >1                   
                }
            }
            else { array = set_array_value(array, j, i, 0, 0, 0); } // wall
        }
    }

    let image = array_to_image(array); 
    image.save(name).unwrap();
}

//https://stackoverflow.com/questions/56762026/how-to-save-ndarray-in-rust-as-image
fn array_to_image(arr: Array3<u8>) -> RgbImage
{
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn set_array_value(mut arr: Array3<u8>, i: usize, j: usize, val1: u8, val2: u8, val3: u8) -> Array3<u8>
{
    arr[[i, j, 0]] = val1;
    arr[[i, j, 1]] = val2;
    arr[[i, j, 2]] = val3;

    arr
}

//Pixel should be red if touched by two or more reds
pub fn is_false_blue(mtx: &[Vec<u8>], x: usize, y: usize) -> bool
{
    let mut sum_red: u8 = 0;
    
    if x != 0 && y != 0 && x < mtx[0].len()-1 && y < mtx[0].len()-1
    {
        if mtx[x][y+1] == 1 { sum_red+=1; }
        if mtx[x][y-1] == 1 { sum_red+=1; }
        if mtx[x+1][y] == 1 { sum_red+=1; }
        if mtx[x-1][y] == 1 { sum_red+=1; }
    }
    sum_red >= 2
}
