//Reads the color and translates to buffer position and applies color based on matrix value
pub fn _2d_to_flat_color(mtx: &[Vec<u8>], size: usize, n: usize, b: usize) -> u32
{
    let red = 16711680;
    let blue = 255;
    let black = 0;
    let white = 16777215;

    let x = n%size;
    let y = b%size;

    //Set colors for animation (different than for the image since the animation is based on a 32 bit frame buffer)
    if mtx[x][y] < u8::MAX
    {
        if mtx[x][y] == 0 { white } //space
        else if mtx[x][y] == 1 { red } //travelled once
        else
        {
            if crate::toimage::is_false_blue(mtx, x, y) { red } //travelled >1 but in the solution path
            else { blue } //travelled >1
        }
    }
    else { black } //wall
}
