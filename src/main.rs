use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() -> Result<(), &'static str> {
    let day_fns = [
        day_01::day_01,
        day_02::day_02,
        day_03::day_03,
        day_04::day_04,
    ];

    if let Some(day_str) = env::args().skip(1).next() {
        let day: usize = day_str.parse().map_err(|_| {
            let error_msg: &str = "Argument must be a valid day number (integer)";
            error_msg.into()
        })?;

        let func = day
            .checked_sub(1)
            .and_then(|idx| day_fns.get(idx))
            .ok_or_else(|| "Day not implemented or invalid.")?;

        // 2. Call the function and use '?' to propagate its Result!
        func();

        Ok(())
    } else {
        Err("Run using 'cargo run -- 1' to execute day 1").into()
    }
}
