mod fs;

use fs::walk_dir;

fn main() {
    let search_dir = "/Users/akash/Documents/images/";
    let files = walk_dir(search_dir, true);
    // let file = files.get(0).unwrap();
    println!("There are: {} files.", files.len())
}
