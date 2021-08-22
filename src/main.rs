use std::time::Instant;
use std::io;
mod generate;
mod toimage;
mod solve;

fn main()
{
    println!("----------------------------------------------------");
    println!("-----------------Maze Cruncher v1.0-----------------");
    println!("----------------------------------------------------");


    let mut size: usize = 75;
    let mut show_animation = true;
    let mut anim_scale = 4;
    let mut anim_speed = 2;


    println!("Use default settings?");
    println!("y/n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let mut trimmed: String = input.trim().to_string();

    if trimmed == "n"
    {
        println!();
        println!("Enter a maze size: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let mut trimmed: String = input.trim().to_string();
        match input.trim().parse::<u32>() {
            Ok(i) => size = i as usize,
            Err(..) => println!("Invalid Input: {}", trimmed),
        };

        println!("A solution image will be saved to this directory");

        if size <= 2048
        {
            println!("Do you want to display the solution animation? (takes longer but looks cool)");
            println!("y/n: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            let mut trimmed = input.trim().to_lowercase();
            show_animation = trimmed == "y";

            if show_animation
            {
                println!("Enter animation scale: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read");
                trimmed = input.trim().to_string();
                match trimmed.parse::<u32>() {
                    Ok(i) => anim_scale = i as usize,
                    Err(..) => println!("Invalid Input: {}", trimmed),
                };

                println!("Enter animation speed: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read");
                trimmed = input.trim().to_string();
                match trimmed.parse::<u32>() {
                    Ok(i) => anim_speed = i as usize,
                    Err(..) => println!("Invalid Input: {}", trimmed),
                };
            }
        }
        else 
        {
            show_animation = false;
        }

    }

    println!("Initializing");

    let before = Instant::now();
    generate::generate_maze(size, show_animation, anim_scale, anim_speed); //Solver is called from within generator
    println!("Elapsed time: {:.2?}", before.elapsed());

    println!("Done!");
    println!("Solved and unsolved images of this maze saved to this directory");
    println!("Press enter to continue");
    io::stdin().read_line(&mut input);
}
