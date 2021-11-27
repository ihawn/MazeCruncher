use minifb::Window;
use rand::Rng;


pub fn binary_tree(mut window: Window, buff_size: usize, mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_speed_mult: usize) -> (Vec<Vec<u8>>, Window)
{
    let x = 1; 
    let y = 1;
    mtx[x][y] = 0;

    let itt: u128 = (size as u128)*(size as u128);
    let mut k = 0;
    for i in (1..size-1).step_by(2)
    {
        for j in (1..size-1).step_by(2)
        {
            window = crate::utils::update_window(window, show_animation, k, &mtx, size, anim_speed_mult, buff_size);
            crate::utils::update_gen_counter(itt, k);
            k += 1;

            let choice = rand::thread_rng().gen_range(0..2);

            if choice == 0
            {
                if i+1 != size-1
                {
                    mtx[i+1][j] = 0;
                }
                else if j > 1
                {
                    mtx[i][j-1] = 0;
                }
            }
            else
            {
                if j > 1
                {
                    mtx[i][j-1] = 0;
                }
                else if i+1 != size-1
                {
                    mtx[i+1][j] = 0;
                }
            }

            mtx[i][j] = 0;
        }
    }
    (mtx, window)
}
