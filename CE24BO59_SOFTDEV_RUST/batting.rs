//importing fmt module from the Rust's standard library which allows us to format strings and implement custom display methods

use std::fmt;
use std::io::{self, Write};

// Struct representing a single over
#[derive(Clone)]
struct Over {
    // balls is a vector of integers storing the number of runs scored on each ball in the over
    balls: Vec<i32>, 
    // is_doubled stores the boolean value in a vector which indicates whether or not the runs in each ball in that over will be doubled or not
    is_doubled: Vec<bool>, 
}

// Constructor of Over struct
impl Over {
    fn new(balls: Vec<i32>, is_doubled: Vec<bool>) -> Over {
        Over { balls, is_doubled }
    }
    
    // This method is used to check if the over is a maiden (no runs scored in the over).
    // The .all method is used to check if all the elements stored in the vector balls are 0 or not.
    fn is_maiden(&self) -> bool {
        self.balls.iter().all(|&runs| runs == 0)
    }

    // Boundaries are defined as 4 runs or 6 runs; thus, similar to the is_maiden method, this method checks whether the values in the balls vector are >= 4.
    fn all_boundaries(&self) -> bool {
        self.balls.iter().all(|&runs| runs >= 4)
    }

    // Iterates through the first two elements of the balls vector to confirm whether or not the first 2 balls are sixes.
    fn first_two_sixes(&self) -> bool {
        self.balls.len() >= 2 && self.balls[0] == 6 && self.balls[1] == 6
    }

    // "            " last 2 balls are sixes.
    fn last_two_sixes(&self) -> bool {
        self.balls.len() >= 6 && self.balls[4] == 6 && self.balls[5] == 6
    }

    // Creates a general format of the over as a string for display as mentioned in the app, and creates the special criteria.
    fn display(&self) -> String {
        let mut result = String::new();
        for (i, &run) in self.balls.iter().enumerate() {
            let boundary_marker = if run >= 4 { "*" } else { "" };
            let double_marker = if self.is_doubled[i] { "x2" } else { "" };
            result.push_str(&format!("{}{}{}", boundary_marker, run, double_marker));
            if i < self.balls.len() - 1 {
                result.push_str(", ");
            }
        }
        format!("({})", result)
    }

    // This function calculates total runs for the over.
    // The .zip function pairs the balls' runs with the fact whether it will be doubled or not.
    // If doubled, it multiplies the runs by 2 and otherwise it takes the original runs using the .sum() function.
    // Sums up all the runs.
    fn total_runs(&self) -> i32 {
        self.balls
            .iter()
            .zip(&self.is_doubled)
            .map(|(&run, &doubled)| if doubled { run * 2 } else { run })
            .sum()
    }
}

// Struct representing the entire game
// overs is a vector of regular overs and extra_overs is a vector of overs that have the twists.
struct Game {
    overs: Vec<Over>,
    extra_overs: Vec<Over>,
}

// Constructor for the Game structure initializes an empty game with no overs which can take input for the overs and apply the strategy.
impl Game {
    fn new() -> Game {
        Game {
            overs: Vec::new(),
            extra_overs: Vec::new(),
        }
    }
    
    // Adds a regular over to a game with no twist.
    fn add_over(&mut self, over: Over) {
        self.overs.push(over);
    }
    
    // Adds an extra over (when required) to the game.
    fn add_extra_over(&mut self, over: Over) {
        self.extra_overs.push(over);
    }

    // This function is used to apply special powers to the game based on twists.
    fn apply_twists(&mut self) {
        let mut i = 0;
        while i < self.overs.len() {
            let over = &self.overs[i];

            // If all the overs are boundaries, the next 2 overs' runs are doubled.
            if over.all_boundaries() {
                // Next two overs are doubled
                for j in i+1..i+3 {
                    if j < self.overs.len() {
                        self.overs[j].is_doubled = vec![true; 6];
                    }
                }
            }

            // If the first 2 balls are sixes then the remaining balls of that over are doubled, unless they are boundaries.
            if over.first_two_sixes() {
                // Remaining balls of the over are doubled
                for k in 2..6 {
                    if over.balls[k] < 4 {
                        self.overs[i].is_doubled[k] = true;
                    }
                }
            }

            // If the last two balls are sixes (and the first four weren't all boundaries), the next over's runs are doubled.
            if over.last_two_sixes() && !over.all_boundaries() {
                // Next over is doubled
                if i + 1 < self.overs.len() {
                    self.overs[i+1].is_doubled = vec![true; 6];
                }
            }

            // Handle extra overs if necessary.
            // If the 10th over has all boundaries, two extra overs are added, with all runs doubled.
            if over.all_boundaries() && i == 9 {
                self.extra_overs.push(Over::new(vec![0, 0, 0, 0, 0, 0], vec![true; 6]));
                self.extra_overs.push(Over::new(vec![0, 0, 0, 0, 0, 0], vec![true; 6]));
            }

            i += 1;
        }
    }

    // Calculates the total score of the game by summing up runs of all extra and regular overs.
    fn total_score(&self) -> i32 {
        self.overs.iter().map(|over| over.total_runs()).sum::<i32>() +
            self.extra_overs.iter().map(|over| over.total_runs()).sum::<i32>()
    }

    // Displays the scorecard and displays each over (regular + extra) + final score.
    fn display_scorecard(&self) {
        println!("Scorecard:");
        for (i, over) in self.overs.iter().enumerate() {
            println!("Over {}: {}", i + 1, over.display());
        }
        for (i, over) in self.extra_overs.iter().enumerate() {
            println!("Extra Over {}: {}", i + 1, over.display());
        }
        println!("Final Score: {}", self.total_score());
    }
}

// Helper function to take dynamic input for runs and is_doubled
// Prompts the user to enter the runs for each ball and whether or not the runs will be doubled (1 for double, 0 for no double).
fn input_over() -> Over {
    let mut balls = Vec::new();
    let mut is_doubled = Vec::new();
    
    println!("Enter the runs for each of the 6 balls:");
    for i in 1..=6 {
        print!("Ball {}: ", i);
        io::stdout().flush().unwrap();  // Ensures the prompt is displayed immediately
        let mut run_input = String::new();
        io::stdin().read_line(&mut run_input).expect("Failed to read input");
        let run: i32 = run_input.trim().parse().expect("Invalid input, expected an integer");
        balls.push(run);
    }

    println!("Enter 1 if the run on each ball should be doubled, otherwise enter 0:");
    for i in 1..=6 {
        print!("Ball {} (double runs? 1/0): ", i);
        io::stdout().flush().unwrap();  // Ensures the prompt is displayed immediately
        let mut double_input = String::new();
        io::stdin().read_line(&mut double_input).expect("Failed to read input");
        let double: bool = double_input.trim() == "1";  // If user inputs '1', it's considered as true (double), otherwise false
        is_doubled.push(double);
    }

    Over::new(balls, is_doubled)
}

// Main function creates a new game and allows the user to input overs dynamically
// It applies any twists based on the criteria and then prints the final scorecard.
fn main() {
    let mut game = Game::new();
    
    // Prompt the user to enter the number of overs to be added to the game
    println!("Enter the number of overs to input:");
    let mut num_overs_input = String::new();
    io::stdin().read_line(&mut num_overs_input).expect("Failed to read input");
    let num_overs: usize = num_overs_input.trim().parse().expect("Invalid input, expected an integer");

    // Loop through the number of overs specified by the user and input details for each over
    for i in 1..=num_overs {
        println!("\nEnter details for Over {}:", i);
        let over = input_over();
        game.add_over(over);
    }
    
    // Apply any twists to the overs
    game.apply_twists();
    
    // Display the final scorecard
    game.display_scorecard();
}

