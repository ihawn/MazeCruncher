use std::ptr::null;
use rand::Rng;
extern crate minifb;
use minifb::{Key, Window, WindowOptions};

pub fn solve_maze(mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize)
{
    tremaux(mtx, size, show_animation, anim_scale, anim_speed_mult);
}

fn tremaux(mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize) 
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  buff_size*buff_size];

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

    

    //Start of solving algo
    let start_x = 1;
    let start_y = 1;
    let end_x = size - 2;
    let end_y = size - 2;

    let mut x = start_x;
    let mut y = start_y;

    let mut max = 0;

    let mut counter: u128 = 0;
    loop
    {
        //Display window
        if show_animation && counter % anim_speed_mult as u128 == 0
        {
            let mut n: usize = 0;
            let mut b: usize = 0;
            for i in buffer.iter_mut() 
            {

                *i = _2d_to_flat_color(&mtx, size, n, b, anim_scale);

               
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
        mtx[x][y] += 1;
        
        
        let coordinate = traverse_maze(&mtx, x, y);

        if is_dead_end(&mtx, x, y, 0)
        {
            mtx[x][y] += 1;
        }

        x = coordinate[0];
        y = coordinate[1];

          
        if x == end_x && y == end_y
        {  
            println!("Solved");
            break;
        }

        //Counter
        let prod = (x+1)*(y+1);
        if prod > max
        {
            max = prod;
            println!("Solving Maze: {}/{}", prod, size*size);
        }

        counter += 1;

    }

    crate::toimage::mtx_to_img(&mtx, size, "solved.png".to_string());

}

fn traverse_maze(mtx: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<usize>
{
    let mut coord = vec![0, 0];

    //First check if any untravelled paths remain
    //0 = not travelled on, 1 = travelled on once, 2 = travelled on twice, u8::MAX = wall
    let mut trav = check_for_traveled(mtx, x, y, 0); 
    let mut k = 0;

    while trav == 0
    {
        k+=1;
        trav = check_for_traveled(mtx, x, y, k); 
    }


    match trav
    {
        1 => coord = vec![x, y-1],
        2 => coord = vec![x+1,y],
        3 => coord = vec![x, y+1],
        4 => coord = vec![x-1, y],

        _ => println!("Something terrible has happened"),
    }

    
    return coord;
}

//Returns direction to travel to. 0 = can't travel, 1 = up, 2 = right, 3 = down, 4 = left
fn check_for_traveled(mtx: &Vec<Vec<u8>>, x: usize, y: usize, can_travel_num: u8) -> usize
{
    //look up
    if mtx[x][y-1] <= can_travel_num
    {
        return 1;
    }
    //look right
    else if mtx[x+1][y] <= can_travel_num
    {
        return 2;
    }
    //look down
    else if mtx[x][y+1] <= can_travel_num
    {
        return 3;
    }
    //look left
    else if mtx[x-1][y] <= can_travel_num
    {
        return 4;
    }  
    else
    {
        return 0; 
    }

}

//Checks if at dead end according to relative_num
fn is_dead_end(mtx: &Vec<Vec<u8>>, x: usize, y: usize, relative_num: u8) -> bool
{
    let t = check_for_traveled(mtx, x, y, relative_num);
    if t == 0
    {
        return true;
    }
    else
    {
        return false;    
    }
}

//Reads the color and translates to buffer position and applies color based on matrix value
fn _2d_to_flat_color(mtx: &Vec<Vec<u8>>, size: usize, mut n: usize, mut b: usize, anim_scale: usize) -> u32
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
        if mtx[x][y] == 0
        {
            return white; //space
        }
        else if mtx[x][y] == 1
        {
            return red; //travelled once
        }
        else
        {
            if crate::toimage::is_false_blue(mtx, x, y)
            {
                return red; //travelled >1 but in the solution path
            }
            else 
            {
                return blue; //travelled >1
            }
            
        }
    }
    else
    {
        return black; // wall
    }
}