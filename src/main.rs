use std::time::Instant;
use std::io;
use std::io::{stdin, stdout, Read, Write};
mod generate;
mod toimage;
mod solve;
mod utils;
mod algo_dfs;
mod algo_astar;
mod algo_tremaux;
mod algo_dijkstra;
mod algo_bfs;
mod algo_def;

fn main()
{

    println!(r"
 __  __                 _____                       _               _ 
|  \/  |               /  __ \                     | |             | |
| .  . | __ _ _______  | /  \/_ __ _   _ _ __   ___| |__   ___ _ __| |
| |\/| |/ _` |_  / _ \ | |   | '__| | | | '_ \ / __| '_ \ / _ \ '__| |
| |  | | (_| |/ /  __/ | \__/\ |  | |_| | | | | (__| | | |  __/ |  |_|
\_|  |_/\__,_/___\___|  \____/_|   \__,_|_| |_|\___|_| |_|\___|_|  (_)                                                                
");


    let mut size: usize = 200;
    let mut save_maze = true;
    let mut show_animation = true;
    let mut anim_scale = 4;
    let mut anim_speed = 5;
    let mut algo = 4;
    let mut decimation = 0;


    let use_default = read_bool("Use default settings?".to_string(), "y/n: ".to_string());

    

    if !use_default
    {
        algo = read_int("Select algorithm:".to_string(), "1) Depth First Search\n2) Breadth First Search\n3) Dead End Filling\n4) A*\n5) Dijkstra\n6) Tremaux\n7) All of them!".to_string(), 2);
        decimation = read_int("Select maze decimation probability (0 = perfect maze)".to_string(), "0-100: ".to_string(), 2);
        save_maze = read_bool("Save the solved and unsolved maze?".to_string(), "y/n: ".to_string());
        if save_maze
        {
            println!("A solution image will be saved to this directory");
        }
        size = read_int("Enter a maze size: ".to_string(), "".to_string(), size);
        

        if size <= 2048
        {
            show_animation = read_bool("Do you want to display the solution animation? (takes longer but looks cool)".to_string(), "y/n: ".to_string());

            if show_animation
            {
                anim_scale = read_int("Enter animation scale: ".to_string(), "".to_string(), anim_scale);
                anim_speed = read_int("Enter animation speed: ".to_string(), "".to_string(), anim_speed);
            }
        }
        else 
        {
            show_animation = false;
        }

    }

    println!("Initializing");

    let before = Instant::now();
    generate::generate_and_solve(size, algo, decimation, show_animation, anim_scale, anim_speed, save_maze); //Solver is called from within generator
    println!("Elapsed time: {:.2?}", before.elapsed());

    println!("Done!");
    
    if save_maze
    {
        println!("Solved and unsolved images of this maze were saved to this directory");
    }
    pause();
}

fn read_int(message: String, choices: String, default_value: usize) -> usize
{
    
    let mut input = String::new();
    println!();
    println!("{}", message);
    println!("{}", choices);

    io::stdin().read_line(&mut input).expect("Failed to read");
    let trimmed = input.trim().to_string();
    match trimmed.parse::<u32>() {
        Ok(i) => return i as usize,
        Err(..) => println!("Invalid Input: {}", trimmed),
    };

    default_value
}

fn read_bool(message: String, choices: String) -> bool
{
    let mut input = String::new();
    println!();
    println!("{}", message);
    println!("{}", choices);

    io::stdin().read_line(&mut input).expect("Failed to read");
    let trimmed: String = input.trim().to_string().to_lowercase();
    trimmed == "y"
}

fn pause()
{
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}