use std::io::{self, Write};

// Struct representing a single over
#[derive(Clone)]
struct Over {
    balls: Vec<i32>,          // Runs scored on each ball in the over
    is_doubled: Vec<bool>,     // Indicates if runs on each ball are doubled
}

// Constructor of Over struct
impl Over {
    fn new(balls: Vec<i32>, is_doubled: Vec<bool>) -> Over {
        Over { balls, is_doubled }
    }

    // This method checks if all balls in the over are boundaries (4 or 6 runs).
    fn all_boundaries(&self) -> bool {
        self.balls.iter().all(|&runs| runs >= 4)
    }

    // Checks if the first two balls of the over are sixes.
    fn first_two_sixes(&self) -> bool {
        self.balls.len() >= 2 && self.balls[0] == 6 && self.balls[1] == 6
    }

    // Checks if the last two balls of the over are sixes.
    fn last_two_sixes(&self) -> bool {
        self.balls.len() >= 6 && self.balls[4] == 6 && self.balls[5] == 6
    }

    // Formats the over for display.
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

    // Calculates the total runs for the over, doubling where needed.
    fn total_runs(&self) -> i32 {
        self.balls
            .iter()
            .zip(&self.is_doubled)
            .map(|(&run, &doubled)| if doubled { run * 2 } else { run })
            .sum()
    }
}

// Struct representing the entire game
struct Game {
    overs: Vec<Over>,
    extra_overs: Vec<Over>,
}

// Constructor for the Game structure initializes an empty game with no overs.
impl Game {
    fn new() -> Game {
        Game {
            overs: Vec::new(),
            extra_overs: Vec::new(),
        }
    }
    
    // Adds a regular over to a game.
    fn add_over(&mut self, over: Over) {
        self.overs.push(over);
    }

    // This function applies special powers to the game based on twists.
    fn apply_twists(&mut self) {
        let mut i = 0;
        while i < self.overs.len() {
            let over = self.overs[i].clone();

            // If all balls are boundaries, double the next two overs' runs.
            if over.all_boundaries() {
                for j in i + 1..i + 3 {
                    if j < self.overs.len() {
                        self.overs[j].is_doubled = vec![true; 6];
                    }
                }
            }

            // If the first two balls are sixes, double remaining balls of this over if not boundaries.
            if over.first_two_sixes() {
                for k in 2..6 {
                    if self.overs[i].balls[k] < 4 {
                        self.overs[i].is_doubled[k] = true;
                    }
                }
            }

            // If last two balls are sixes, double the next over if it exists.
            if over.last_two_sixes() && !over.all_boundaries() {
                if i + 1 < self.overs.len() {
                    self.overs[i + 1].is_doubled = vec![true; 6];
                }
            }

            // If the 10th over is all boundaries, add two extra overs with doubled runs.
            if over.all_boundaries() && i == 9 {
                self.extra_overs.push(Over::new(vec![0; 6], vec![true; 6]));
                self.extra_overs.push(Over::new(vec![0; 6], vec![true; 6]));
            }

            i += 1;
        }
    }

    // Calculates the total score by summing up all overs.
    fn total_score(&self) -> i32 {
        self.overs.iter().map(|over| over.total_runs()).sum::<i32>()
            + self.extra_overs.iter().map(|over| over.total_runs()).sum::<i32>()
    }

    // Displays the scorecard including each over (regular and extra) and final score.
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
fn input_over() -> Over {
    let mut balls = Vec::new();
    let mut is_doubled = Vec::new();
    
    println!("Enter the runs for each of the 6 balls:");
    for i in 1..=6 {
        print!("Ball {}: ", i);
        io::stdout().flush().unwrap();
        let mut run_input = String::new();
        io::stdin().read_line(&mut run_input).expect("Failed to read input");
        let run: i32 = run_input.trim().parse().expect("Invalid input, expected an integer");
        balls.push(run);
    }

    println!("Enter 1 if the run on each ball should be doubled, otherwise enter 0:");
    for i in 1..=6 {
        print!("Ball {} (double runs? 1/0): ", i);
        io::stdout().flush().unwrap();
        let mut double_input = String::new();
        io::stdin().read_line(&mut double_input).expect("Failed to read input");
        let double: bool = double_input.trim() == "1";
        is_doubled.push(double);
    }

    Over::new(balls, is_doubled)
}

// Main function to create a new game, input overs, apply twists, and display the scorecard.
fn main() {
    let mut game = Game::new();
    
    println!("Enter the number of overs to input:");
    let mut num_overs_input = String::new();
    io::stdin().read_line(&mut num_overs_input).expect("Failed to read input");
    let num_overs: usize = num_overs_input.trim().parse().expect("Invalid input, expected an integer");

    for i in 1..=num_overs {
        println!("\nEnter details for Over {}:", i);
        let over = input_over();
        game.add_over(over);
    }
    
    game.apply_twists();
    game.display_scorecard();
}
