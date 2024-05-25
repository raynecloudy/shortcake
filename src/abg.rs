// converted to python. i couldn't figure out how to run python in rust :(
// original file is viewable at scraps/abg.py.md
// this file caused so many errors i almost cried

pub fn read(file: String) {
	let lines: Vec<_> = file.split("\n").collect();
	if lines[0] == "1.0.1" {
		let brightnesslevels = vec!["1", "2", "3", "4", "x"];
		let brightness = ["  ", "░░", "▓▓", "██", "\x1b[0m  \x1b[40m"];
		let colornames = vec!["r", "g", "y", "v", "p", "b", "w", "x"];
		let colors = ["\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m", "\x1b[37m", ""];
		for y in 0..lines.len()-2 {
			let linepixels: Vec<_> = lines[y+2].split(" ").collect();
			for x in 0..linepixels.len()-1 {
				let pixel: Vec<char> = linepixels[x].chars().collect();
				if colornames.contains(&pixel[0].to_string().as_str()) {
					print!("{}", colors[colornames.iter().position(|&r| r == pixel[0].to_string()).unwrap()]);
				} else {
					print!("\x1b[37m");
				}
				if brightnesslevels.contains(&pixel[1].to_string().as_str()) {
					print!("{}", brightness[brightnesslevels.iter().position(|&r| r == pixel[1].to_string()).unwrap()]);
				} else {
					print!("??");
				}
			}
			println!("");
		}
	}
}