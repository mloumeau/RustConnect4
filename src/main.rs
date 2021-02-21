//This is for getting input from the user.
use std;


fn display_board(game_array: & mut [[char;6];8]) {
    
    let mut j = 0;
    let mut i;
    
    //Show the column choices
    println!("    1     2     3     4     5     6     7     8\n");

    //Rows
    while j < 6 {
        //Reset i
        i = 0;
        print!("|");
        //Columns
        while i < 8{

            //Display the game
            print!("|  {}  ", game_array[i][j]);

            //If it's the last column, close it off
            if i == 7{
                print!("||");
            }

            i = i + 1; 
        }

        //After each row, separate with ---
        println!("\n---------------------------------------------------");
        j = j + 1;
    }

    //Display the base of the board
    for _ in 0..3{
        println!("||                                               ||");
    }
}

fn drop_in_available_slot(mut game_array: & mut [[char;6];8], col: usize, symbol: char, count: i8){

    //Reverse the order so the bottom row is what's checked first.
    //Since usize is >= 0, we have to do 0-6, and check if it's 0 (full)
    for i in (0..=6).rev(){

        //If full, try again
        if i == 0{
            //If every slot is full, quit and output "Draw!"
            for x in 0..=7{
                if game_array[x][0] == 'X' || game_array[x][0] ==  'O'{
                    if x == 7{
                        println!("Draw!");
                        std::process::exit(0);
                        }
                    }
                    else{
                        break;
                    }
                }
            println!("Column is Full!");
            ask_user(count, &mut game_array);
        }

        //Else, place the chip, show the result, check if there's a winner.
        if game_array[col][i-1] == ' '{
            game_array[col][i-1] = symbol;
            display_board(& mut game_array);
            
            check_winner(& mut game_array, symbol, i-1, col);
            println!("\n\n");
            break;
        }

    }

}

fn get_symbol(count: i8) -> char{
    let symbol;

    //If count is odd, it's player 1's turn
    if count % 2 == 1{
        symbol = 'X';
    }

    //If count is even, it's player 2's turn
    else{
        symbol = 'O';
    }
    //Return symbol
    symbol
}

fn ask_user(count: i8, mut game_array: & mut [[char;6];8]) -> i8{
    
    //This allows for the user to input something
    //incorrectly, but the symbol will still be correct.
    let mut error_count = count;
    
    //This let's us know who's turn it is.
    let symbol = get_symbol(count);

    //Get the input from the user (String).
    let mut input_text = String::new();
    println!("Enter Slot to Drop (1-8) :");
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    //Match statements are like switch statements.
    match input_text.trim().parse::<usize>() {
        Err(..) => {
            println!("this was not an integer: {}", input_text);
            //Return count - 1, since count
            //gets incremented when returned.
            error_count = count - 1
        },

        _ => {
            //Convert the string to a usize in order to
            //be able to use it for array indexing.

            //By creating this variable inside
            //the else of our match statement,
            //it confirms that it is an integer.

            let input_int: usize = input_text
                .trim()
                .parse()
                .expect("Nope");
            match input_int{
                //This means range(1, 9)
                1..=8 => drop_in_available_slot(&mut game_array, input_int-1, symbol, count),
                _ =>  {
                    println!("Not a Valid Slot: {}", input_text);
                    //Return count - 1, since count
                    //gets incremented when returned.
                    error_count = count - 1
                }
            }

        }
    };
    //If error, the count will remain the same
    //If no error, the count will increment.
    //Return the result
    error_count + 1
}

fn check_winner(game_array: & mut [[char;6];8], _symbol: char, row: usize, col: usize){

    //The Only Vertical Case

    if row < 3 
    && game_array[col][row] == game_array[col][row+1] 
    && game_array[col][row+1] == game_array[col][row+2] 
    && game_array[col][row+2] == game_array[col][row+3]{
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //All Horizontal Cases

    //Right-Most Chip Wins it

    else if col > 2 
    && game_array[col][row] == game_array[col-1][row] 
    && game_array[col-1][row] == game_array[col-2][row] 
    && game_array[col-2][row] == game_array[col-3][row]{
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Second-to-Left Chip Wins it

    else if col > 0 && col < 6 
    && game_array[col][row] == game_array[col-1][row] 
    && game_array[col][row] == game_array[col+1][row] 
    && game_array[col+1][row] == game_array[col+2][row] {
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Second-to-Right Chip Wins it

    else if col > 1 && col < 7 
    && game_array[col-1][row] == game_array[col-2][row] 
    && game_array[col][row] == game_array[col-1][row] 
    && game_array[col][row] == game_array[col+1][row] {
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Left-Most Chip Wins it

    else if col < 5 
    && game_array[col][row] == game_array[col+1][row] 
    && game_array[col+1][row] == game_array[col+2][row] 
    && game_array[col+2][row] == game_array[col+3][row]{
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }


    //North-West to South-East Diagonal Cases

    //Left-Most Chip Wins it

    else if col < 5 
    && row < 3 
    && game_array[col][row] == game_array[col+1][row+1] 
    && game_array[col+1][row+1] == game_array[col+2][row+2] 
    && game_array[col+2][row+2] == game_array[col+3][row+3]{
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Second-to-Left Most Chip Wins it

    else if col > 0 && col < 6 
    && row > 0 && row < 4 
    && game_array[col][row] == game_array[col-1][row-1] 
    && game_array[col][row] == game_array[col+1][row+1] 
    && game_array[col+1][row+1] == game_array[col+2][row+2] {
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Second-to-Right Chip Wins it

    else if col > 1 && col < 7 
    && row > 1 && row < 5 
    && game_array[col-1][row-1] == game_array[col-2][row-2] 
    && game_array[col][row] == game_array[col-1][row-1] 
    && game_array[col][row] == game_array[col+1][row+1] {
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Right-Most Chip Wins it

    else if col > 2 
    && row > 2 
    && game_array[col][row] == game_array[col-1][row-1] 
    && game_array[col-1][row-1] == game_array[col-2][row-2] 
    && game_array[col-2][row-2] == game_array[col-3][row-3]{
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //South-West to North-East Diagonal Cases

    //Left-Most Chip Wins it

    else if col < 5 
    && row > 2 
    && game_array[col][row] == game_array[col+1][row-1] 
    && game_array[col+1][row-1] == game_array[col+2][row-2] 
    && game_array[col+2][row-2] == game_array[col+3][row-3]{
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Second-to-Left Most Chip Wins it

    else if col > 0 && col < 6 
    && row > 1 && row < 5 
    && game_array[col][row] == game_array[col-1][row+1] 
    && game_array[col][row] == game_array[col+1][row-1] 
    && game_array[col+1][row-1] == game_array[col+2][row-2] {
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Second-to-Right Chip Wins it

    else if col > 1 && col < 7 
    && row > 0 && row < 4 
    && game_array[col-1][row+1] == game_array[col-2][row+2] 
    && game_array[col][row] == game_array[col-1][row+1] 
    && game_array[col][row] == game_array[col+1][row-1] {
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }

    //Right-Most Chip Wins it

    else if col > 2 && row < 3
    && game_array[col][row] == game_array[col-1][row+1] 
    && game_array[col-1][row+1] == game_array[col-2][row+2] 
    && game_array[col-2][row+2] == game_array[col-3][row+3]{
        println!("{}'s Win!", _symbol);
        std::process::exit(0);
    }
}

                

fn main() {

    //Count determines if it should be an 'X' or an 'O'
    let mut count = 1;

    //This is how to create an (8x6) two-dimensional array
    //filled with ' '.
    let mut game_array:[[char;6];8] = [[' ';6];8];

    //Show the empty board.
    display_board(& mut game_array);

    //This is a while true loop. We can do this
    //because the program exits when someone wins or draws
    loop {
        count = ask_user(count, & mut game_array);
    }
}
