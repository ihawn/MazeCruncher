use rand::Rng;
use minifb::Window;

//Growing Tree Algorithm for implementing a "perfect" maze i.e. only one solution
pub fn growing_tree(mut window: Window, buff_size: usize, mut mtx: Vec<Vec<u8>>, size: usize, decimation: usize, show_animation: bool, anim_speed_mult: usize, save_maze: bool) -> (Vec<Vec<u8>>, Window)
{


    //Maze generation init
    println!("Generating Maze");
    let factor = decimation;
    let cell_size = (size - 1)/2;
    let mut x = rand::thread_rng().gen_range(0..cell_size)*2 + 1;
    let mut y = rand::thread_rng().gen_range(0..cell_size)*2 + 1;
    let mut x_list = Vec::new();
    let mut y_list = Vec::new();
    x_list.push(x); //push the starting point onto the list
    y_list.push(y);
    mtx[x][y] = 0;

    let s: u128 = size as u128;
    let itt: u128 = (s - 3)*(s + 1)/4;
    for k in 0..itt
    {
        if k%(itt/100) == 0
        {
            let m = 100*k/itt + 1;
            println!("Generating Maze: {}%", m);
        }

        window = crate::utils::update_window(window, show_animation, k, &mtx, size, anim_speed_mult, buff_size);

        x = *x_list.last().unwrap();
        y = *y_list.last().unwrap();


        //Check if current tile can go farther. If not, remove it from the list and try the next one
        let mut travel = travel_to(&mtx, size, &x, &y);
        while travel.is_empty() && !x_list.is_empty()
        {
            x_list.pop();
            y_list.pop();
            if !x_list.is_empty()
            {
                x = *x_list.last().unwrap();
                y = *y_list.last().unwrap();
            }
            travel = travel_to(&mtx, size, &x, &y);
        }

        let n = rand::thread_rng().gen_range(0..travel.len());

        if !x_list.is_empty()
        {
            //movement up, right, down, or left
            match travel[n]
            {
                1 => {mtx[x][y-2] = 0;
                    mtx[x][y-1] = 0;
                    x_list.push(x);
                    y_list.push(y-2)},

                2 => {mtx[x+2][y] = 0;
                    mtx[x+1][y] = 0;
                    x_list.push(x+2);
                    y_list.push(y)},

                3 => {mtx[x][y+2] = 0;
                    mtx[x][y+1] = 0;
                    x_list.push(x);
                    y_list.push(y+2)},

                4 => {mtx[x-2][y] = 0;
                    mtx[x-1][y] = 0;
                    x_list.push(x-2);
                    y_list.push(y)},

                _ => println!("Something terrible has happened"),
            }
        }
    }

    //Prevent a perfect maze for decimation factor > 0
    if factor > 0 { mtx = crate::generate::decimate_maze(mtx, size, factor); }

    //print_matrix(&mtx, size);
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "unsolved.png".to_string());
    }

    println!("Maze Generation Complete");
    (mtx, window)
}

//Function to determine if current cell has any adjacent, non-visited cells
fn travel_to(mtx: &[Vec<u8>], size: usize, x: &usize, y: &usize) -> Vec<i32>
{
    let mut dirs = vec![1, 2, 3, 4];

    if *y == 1 || mtx[*x][*y-2] == 0 { dirs = remove_value(dirs, 1); } //look up   
    if *x == size - 2 || mtx[*x+2][*y] == 0 { dirs = remove_value(dirs, 2); } //look right
    if *y == size - 2 || mtx[*x][*y+2] == 0 { dirs = remove_value(dirs, 3); } //look down    
    if *x == 1 || mtx[*x-2][*y] == 0 { dirs = remove_value(dirs, 4); } //look left

    dirs
}

//Function to remove value from vector
fn remove_value(mut values: Vec<i32>, value: i32) -> Vec<i32>
{
    for i in 0..values.len()
    {
        if values[i] == value
        {
            values.remove(i);
            break;
        }
    }

    values
}