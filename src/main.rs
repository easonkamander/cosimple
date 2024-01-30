mod guard;
mod print;
mod solve;
mod types;

fn pipeline() -> Result<String, String> {
    let path = std::env::args().nth(1).ok_or("Please enter a file")?;
    let text = std::fs::read(path).map_err(|err| format!("Filesystem Failure - {}", err))?;
    let file = types::File::try_from(text)?;
    let tree = guard::Guard::from(&file);
    let hats = solve::File::solve(&file, &tree).map_err(|_| "Unguarded Recursion")?;
    let rslt = print::Index::make(&file, &tree, &hats);
    Ok(rslt.prints())
}

fn main() {
    match pipeline() {
        Ok(text) => println!("{}", text),
        Err(err) => eprintln!("Error: {}", err),
    }
}
