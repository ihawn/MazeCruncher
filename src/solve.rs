extern crate minifb;
use minifb::{Window, WindowOptions};

pub fn solve_maze(mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize, save_maze: bool) 
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  1];

    let mut window = Window::new(
            "Maze!",
            0,
            0,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    if show_animation
    {
        buffer = vec![0;  buff_size*buff_size];

        window = Window::new(
            "Maze!",
            buff_size,
            buff_size,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    }

    println!("Got this far");

    //Solver init
    let start_x = 1;
    let start_y = 1;
    let end_x = size - 2;
    let end_y = size - 2;

    let mut x = start_x;
    let mut y = start_y;

    let mut max = 0;

    let counter: u128 = 0;
    loop
    {
        //Display window
        if show_animation && counter % anim_speed_mult as u128 == 0
        {
            let mut n: usize = 0;
            let mut b: usize = 0;
            for i in buffer.iter_mut() 
            {
        
                *i = crate::utils::_2d_to_flat_color(&mtx, size, n, b);
        
               
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


      
        //Solver
        let p: (Vec<Vec<u8>>, usize, usize) = crate::algo_tremaux::tremaux(mtx, x, y);

        mtx = p.0;
        x = p.1;
        y = p.2;


        //Counter
        let prod = (x+1)*(y+1);
        if prod > max
        {
            max = prod;
            println!("Solving Maze: {}/{}", prod, size*size);
        }

        if x == end_x && y == end_y
        {  
            println!("Solved");
            break;
        }

    }

    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved.png".to_string());
    }
}