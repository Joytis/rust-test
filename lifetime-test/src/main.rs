fn main() {
    'search: for y in 0..20 {
        'explicit: for x in 0..20 {
            println!("Your x is {} and your y is {}", x, y);
            if x == 10 && y == 1 {
                break 'explicit;
            }

            if x == 15 && y == 18 {
                break 'search;
            }
        }
    }
}