use rand::Rng;
use minifb::Window;
use std::cmp::max;


pub fn eller(mut window: Window, buff_size: usize, mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_speed_mult: usize) -> (Vec<Vec<u8>>, Window)
{
    let mut mirror: Vec<Vec<u64>> = vec![vec![0; size]; size];
    let x = 1; 
    let y = 1;
    mtx[x][y] = 0;
    
    let itt: u128 = (size as u128)*(size as u128);
    let mut k = 0;
    let mut id: u64 = 1;
    let mut row_locs: Vec<usize> = vec![];
    for i in (1..size-2).step_by(2) { row_locs.push(i); }

    for j in (1..size-1).step_by(2)
    {
        for j in 0..size
        {
            for i in 0..size
            {
                print!(" {} ", mirror[i][j])
            }
            println!("")
        }

        //initialize current mirror matrix row
        for i in (1..size-1).step_by(2)
        {
            if mirror[i][j] == 0
            {
                mirror[i][j] = id;
                id+=1;
                mtx[i][j] = 0;
            }

            k+=1;
            crate::utils::update_gen_counter(itt, k);
            window = crate::utils::update_window(window, show_animation, k, &mtx, size, anim_speed_mult, buff_size);
        }

        //lateral join   
        let join_count = rand::thread_rng().gen_range(0..(size-1)); //Determine how many locations not to join
        let mut locs = row_locs.clone();

        //remove locations not to join
        let mut p = locs.len();
        let mut u = 0;
        while p > 0 && u < join_count
        {
            locs.remove(rand::thread_rng().gen_range(0..locs.len()));
            p-=1;
            u+=1;
        }

        println!("{}", join_count);
        for i in 0..locs.len()
        {   
            let x = locs[i];
            if mirror[x][j] != mirror[x+2][j]
            {
                let old_id = mirror[x][j];
                
                mirror[x][j] = mirror[x+2][j];
                mirror[x+1][j] = mirror[x+2][j];
                mtx[x][j] = 0;
                mtx[x+1][j] = 0;
                mtx[x+2][j] = 0;

                //Match set IDs
                let mut n = x-1;
                while n > 0 && mirror[n][j] == old_id
                {
                    mirror[n][j] = mirror[x+2][j];         
                    mtx[n][j] = 0;
                    n-=1;
                }
            }

            k+=1;
            crate::utils::update_gen_counter(itt, k);
            window = crate::utils::update_window(window, show_animation, k, &mtx, size, anim_speed_mult, buff_size);
        }

        let mut sets: Vec<(usize, usize)> = vec![];    
        let mut set: (usize, usize) = (0, 0);
        for i in 1..size-1
        {
            //set start
            if i == 1 || mtx[i-1][j] == u8::MAX { set.0 = i; }

            //set end
            if mtx[i+1][j] == u8::MAX { set.1 = i+1; sets.push(set); set = (0, 0); }
        }

        //vertical join
        for s in sets
        {
            let join_count = rand::thread_rng().gen_range(1..max((s.1 - s.0)/2, 2));
            let mut join_pos: Vec<usize> = vec![];
            for i in (s.0..s.1).step_by(2) { join_pos.push(i); } //Set join init

            let mut l = 0;
            while l < join_count && join_pos.len() > 1 { join_pos.remove(rand::thread_rng().gen_range(0..join_pos.len())); l+=1;} //Finalize vertical join positions
            
            
            for join in join_pos
            {
                if j+2 < size && mirror[join][j+2] != mirror[join][j]
                {
                    mirror[join][j+1] = mirror[join][j];
                    mirror[join][j+2] = mirror[join][j];
                    mtx[join][j+1] = 0;
                    mtx[join][j+2] = 0;
                }
            }

            k+=1;
            crate::utils::update_gen_counter(itt, k);
            window = crate::utils::update_window(window, show_animation, k, &mtx, size, anim_speed_mult, buff_size);
        }
    }



    (mtx, window)
}