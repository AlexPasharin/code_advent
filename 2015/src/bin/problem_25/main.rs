static INITIAL_CODE: u64 = 20151125;
static ROW: u32 = 2978;
static COLUMN: u32 = 3083;

static MULTIPLIER: u64 = 252533;
static MAX: u64 = 33554393;

fn main() {
    println!("{}", code(ROW, COLUMN)); //2650453
}

fn code(row: u32, column: u32) -> u64 {
    let position = grid_position_ordinal_number(row, column);

    (INITIAL_CODE * power(position - 1)) % MAX
}

fn grid_position_ordinal_number(row: u32, column: u32) -> u32 {
    let diagonal_starting_point_row = row + column - 1;

    diagonal_starting_point_row * (diagonal_starting_point_row - 1) / 2 + column
}

fn power(exponential: u32) -> u64 {
    if exponential == 0 {
        return 1;
    }

    if exponential % 2 == 0 {
        let half_power = power(exponential / 2);

        (half_power * half_power) % MAX
    } else {
        (power(exponential - 1) * MULTIPLIER) % MAX
    }
}
