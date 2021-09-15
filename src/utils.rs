use minifb::{Window, WindowOptions};

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

//Window buffer update
pub fn update_buffer(mtx: &[Vec<u8>], size: usize, mut buffer: Vec<u32>) -> Vec<u32>
{
    let mut n: usize = 0;
    let mut b: usize = 0;
    for i in buffer.iter_mut() 
    {
        *i = _2d_to_flat_color(mtx, size, n, b);
   
        //"unflatten" the buffer vector
        n+=1;  
        if n%(size) == 0
        {
            n = 0;
            b += 1;
        }              
    }

    buffer
}


pub fn window_init(size: usize, label: &str) -> Window
{
    Window::new(
        label,
        size,
        size,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    })
}


pub fn update_counter(mut max: usize, x: usize, y: usize, size: usize) -> usize
{
    let prod = (x+1)*(y+1);
    if prod > max + size
    {
        max = prod;
        println!("Solving Maze: {}/{}", prod, size*size);
    }

    max
}