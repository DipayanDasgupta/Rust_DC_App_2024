//importing fmt module from the Rust's standard library which allows us to format strings and implement custom display methods
use std::fmt;

// Structrepresenting a single over
#[derive(Clone)]
struct Over {
//balls is a vector of integers storing the number of runs scored on each ball in the over
    balls: Vec<i32>, 
//is_doubled stores the boolean value in a vector which indicates whether or not the runs in each ball in that over will be doubled or not
    is_doubled: Vec<bool>, 
//constructor of over struct (instance of over struct with same name which initializes diff conditions for diff inputs of balls and isDoubled)
impl Over {
    fn new(balls: Vec<i32>, is_doubled: Vec<bool>) -> Over {
        Over { balls, is_doubled }
    }
//this method is used to check if the over is a maiden(no overs scored in the over). The .all method is used to check if all the elements stored in the vector balls is 0 or not
    fn is_maiden(&self) -> bool {
        self.balls.iter().all(|&runs| runs == 0)
    }
//boundaries are defined as 4 runs or 6 runs thus similar to the is_maiden method this method checks whether the values in the balls vector are >=4
    fn all_boundaries(&self) -> bool
        self.balls.iter().all(|&runs| runs >= 4)
    }
//iterates thru the first two elements of the balls vector to confirm whether or not the first 2 balls are sixes.    

    fn first_two_sixes(&self) -> bool {
        self.balls.len() >= 2 && self.balls[0] == 6 && self.balls[1] == 6
    }
// "            " last  2 balls are sixes   

    fn last_two_sixes(&self) -> bool {
        self.balls.len() >= 6 && self.balls[4] == 6 && self.balls[5] == 6
    }

// creates a general format of the over as a string for display as mentioned in the app, and creates the special criteria
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

//this function calculates total runs for the over, the .zip function pairs the balls runs with the fact whether it will be doubled or not, if doubled it multiplies the runs by 2 and otherwise it takes the original runsns using the .sum() func
//sums up all the ru
    fn total_runs(&self) -> i32 {
        self.balls
            .iter()
            .zip(&self.is_doubled)
            .map(|(&run, &doubled)| if doubled { run * 2 } else { run })
            .sum()
    }
}

// This defines the structure representing the entire game
//overs is a vector of regular overs and extra_overs is a vector of overs that have the twists
struct Game {
    overs: Vec<Over>,
    extra_overs: Vec<Over>,
}

//this constructor for the Game Structure initializes an empty game with now overs which can take input for the overs and apply the strategy
impl Game {
    fn new() -> Game {
        Game {
            overs: Vec::new(),
            extra_overs: Vec::new(),
        }
    }
//adds a regular over to a game with no twist
    fn add_over(&mut self, over: Over) {
        self.overs.push(over);
    }
//adds an extra over(when required) to the game
    fn add_extra_over(&mut self, over: Over) {
        self.extra_overs.push(over);
    }

// This function is used to apply special powers to the game based on twists
    fn apply_twists(&mut self) {
        let mut i = 0;
        while i < self.overs.len() {
            let over = &self.overs[i];
//if all the overs are boundaries the next 2 over's runs are doubled            
            if over.all_boundaries() {
                // Next two overs are doubled
                for j in i+1..i+3 {
                    if j < self.overs.len() {
                        self.overs[j].is_doubled = vec![true; 6];
                    }
                }
            }
//if the first 2 balls are sixes then the remaining balls of that over are doubled , unless they are boundaries            
            
            if over.first_two_sixes() {
                // Remaining balls of the over are doubled
                for k in 2..6 {
                    if over.balls[k] < 4 {
                        self.overs[i].is_doubled[k] = true;
                    }
                }
            }
//if the last two balls are sixes (and the first four weren't all boundaries), the next over's runs are doubled.

            if over.last_two_sixes() && !over.all_boundaries() {
                // Next over is doubled
                if i + 1 < self.overs.len() {
                    self.overs[i+1].is_doubled = vec![true; 6];
                }
            }

            // Handle extra overs if necessary
//If the 10th over has all boundaries, two extra overs are added, with all runs doubled.            
            if over.all_boundaries() && i == 9 {
                self.extra_overs.push(Over::new(vec![0, 0, 0, 0, 0, 0], vec![true; 6]));
                self.extra_overs.push(Over::new(vec![0, 0, 0, 0, 0, 0], vec![true; 6]));
            }

            i += 1;
        }
    }

// Calculates the total score of the game by summing up runs of all extra and regular overs
    fn total_score(&self) -> i32 {
        self.overs.iter().map(|over| over.total_runs()).sum::<i32>() +
            self.extra_overs.iter().map(|over| over.total_runs()).sum::<i32>()
    }

// Displays the scorecard and displays each over (regular+extra)+ final score
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
//The main function creates a new game and simulates it by adding 5 example overs.
//It applies any twists and then prints the final scorecard.

fn main() {
    let mut game = Game::new();
    
    // Example game: replace this with dynamic input or random generation
    game.add_over(Over::new(vec![1, 2, 3, 4, 5, 6], vec![false; 6]));  // Regular over
    game.add_over(Over::new(vec![6, 6, 0, 0, 4, 4], vec![false; 6]));  // First two sixes, last four boundaries
    game.add_over(Over::new(vec![0, 0, 1, 2, 3, 4], vec![false; 6]));  // Normal over
    game.add_over(Over::new(vec![0, 0, 0, 0, 0, 0], vec![false; 6]));  // Maiden over
    game.add_over(Over::new(vec![6, 6, 6, 6, 6, 6], vec![false; 6]));  // All sixes (next 2 overs doubled)
    
    // Apply twists
    game.apply_twists();

    // Display the scorecard
    game.display_scorecard();
}

