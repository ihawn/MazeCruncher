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


pub fn euclidean(x: usize, end_x: usize, y: usize, end_y: usize) -> usize
{
    usize::pow(end_x - x, 2) + usize::pow(end_y - y, 2)
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


pub fn update_window(mut window: Window, show_animation: bool, counter: u128, mtx: &[Vec<u8>], size: usize, anim_speed_mult: usize, buff_size: usize) -> Window
{
    if show_animation && counter % anim_speed_mult as u128 == 0
    {
        let mut buffer = vec![0;  buff_size*buff_size];

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

        window
        .update_with_buffer(&buffer, size, size)
        .unwrap();
    }

    window
}

pub fn update_counter(mut max: usize, x: usize, y: usize, size: usize, name: &str) -> usize
{
    let prod = (x+1)*(y+1);
    let m = 100*prod/(size*size)+1;
    if m > max
    {
        max = m;
        println!("Solving Maze with {}: {}%", name, m);
    }

    max
}

pub fn update_gen_counter(itt: u128, counter: u128)
{
    if counter%(itt/100) == 0
    {
        let m = 100*counter/itt + 1;
        println!("Generating Maze: {}%", m);
    }
}

//Fill in matrix values between nodes for the animation/solution image
pub fn fill_between_nodes(mut mtx: Vec<Vec<u8>>, x1: usize, y1: usize, x2: usize, y2: usize, val: u8) -> Vec<Vec<u8>>
{
    let mut sx: usize = x1;
    let mut ex: usize = x2;
    if x2 < x1
    {
        sx = x2;
        ex = x1;
    }
    let mut sy: usize = y1;
    let mut ey: usize = y2;
    if y2 < y1
    {
        sy = y2;
        ey = y1;
    }

    for x in sx..ex+1
    {
        mtx[x][sy] = val;
    }

    for y in sy..ey+1
    {
        mtx[sx][y] = val;
    }

    mtx
}