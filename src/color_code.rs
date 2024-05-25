pub fn color(file: &str, fileex: Option<&str>) -> String {
	let file = file.replace("\t", "\x1b[0m\x1b[2m│\x1b[0m   ");
	match fileex {
		Some("abg") => {
			let lines = file.split("\n").collect::<Vec<_>>();
			let mut linesx = lines.clone();
			linesx.remove(0);
			linesx.remove(0);
			let colored = format!("\x1b[33m{}\n\x1b[0m\x1b[2m\x1b[3m{}\x1b[0m\n\x1b[35m{}", lines[0], lines[1], linesx.join("\n").replace("xx", "\x1b[2mxx\x1b[0m\x1b[35m"));
			return colored;
		},
		Some(&_) | None => return file
	};
}