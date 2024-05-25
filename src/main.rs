extern crate chrono;
extern crate battery;

use std::{io, fs, os::unix::ffi::OsStrExt, env::{current_dir, set_current_dir}, thread, time};
use rand::{seq::SliceRandom, Rng};
use chrono::prelude::*;
use whoami::fallible;

mod abg;
mod file_desc;
mod color_code;

pub fn main() {
	clear();
	checkwidth();
	if whoami::username() == "root" {
		cd("/");
	} else {
		cd(format!("/home/{}", whoami::username()).as_str());
	}
	println!("thank you for using shortcake!");
	clear();
	dashboard();
	writecmdoutput(prettycmd().as_str());
	loop {
		cmdinput();
	}
}

fn dashboard() {
	clear();
	println!("");
	println!("");
	println!("\x1b[36m  ▪       \x1b[35m ██████▓╗ ██▓╗   ██▓╗  ██████▓╗  ██████▓╗  ████████▓╗ \x1b[33m   ████▓╗    ███▓╗    ██▓╗  ██▓╗ ███████▓╗\x1b[36m    ■     \x1b[0m");
	println!("\x1b[36m    ▄     \x1b[35m██▓╔════╝ ██▓║   ██▓║ ██▓╔══██▓╗ ██▓╔═██▓╗ ╚══██▓╔══╝ \x1b[33m ██▓╔═══╝   █████▓╗   ██▓║  ██▓║ ██▓╔════╝\x1b[36m         ▄\x1b[0m");
	println!("\x1b[36m       ■  \x1b[35m██▓║      ██▓║   ██▓║ ██▓║  ██▓║ ██▓║ ██▓║    ██▓║    \x1b[33m██▓╔╝      ██▓╔═██▓╗  ██▓║ ██▓╔╝ ██▓║     \x1b[36m  ▄       \x1b[0m");
	println!("\x1b[36m▪         \x1b[35m╚█████▓   █████████▓║ ██▓║  ██▓║ █████▓╔═╝    ██▓║    \x1b[33m██▓║       ██▓║ ██▓║  █████▓╔═╝  █████▓╗  \x1b[36m       ▪  \x1b[0m");
	println!("\x1b[36m     ▪    \x1b[35m ╚═══██▓╗ ██▓║   ██▓║ ██▓║  ██▓║ ██▓╔═██▓╗    ██▓║    \x1b[33m██▓║      █████████▓╗ ██▓╔═██▓╗  ██▓╔══╝  \x1b[36m        ▀ \x1b[0m");
	println!("\x1b[36m      ▀   \x1b[35m     ██▓║ ██▓║   ██▓║ ██▓║  ██▓║ ██▓║ ╚██▓╗   ██▓║    \x1b[33m╚██▓╗     ██▓╔═══██▓║ ██▓║ ╚██▓╗ ██▓║     \x1b[36m ▪   ▪    \x1b[0m");
	println!("\x1b[36m ▪        \x1b[35m██████▓╔╝ ██▓║   ██▓║ ╚██████▓╔╝ ██▓║  ██▓║   ██▓║    \x1b[33m ╚█████▓╗ ██▓║   ██▓║ ██▓║  ██▓║ ███████▓╗\x1b[36m          \x1b[0m");
	println!("\x1b[36m   ■      \x1b[35m╚══════╝  ╚══╝   ╚══╝  ╚══════╝  ╚══╝  ╚══╝   ╚══╝    \x1b[33m  ╚═════╝ ╚══╝   ╚══╝ ╚══╝  ╚══╝ ╚═══════╝\x1b[36m     ▀   ▪\x1b[0m");
	println!("");
	println!("");
	println!("shortcake is the newest ide in town, with a very simplistic look and modern feel. built by hand from head to toe, \x1b[1mthis project is my live's work thus far\x1b[0m, and");
	println!("  i'm determined to make the best ide for everybody who was supported me on this crazy seven-year ride of my life. i cannot stress this enough, \x1b[1mthank you all.\x1b[0m");
	println!("");
	println!("in shortcake, everything is done by keyboard, no cursor necessary. this might seem like a downgrade from the fancy gui that we have today, but some fellas like");
	println!("  me prefer a very simplistic layout. shortcake brings that to the table, with an extra layer of strawberry flavored \x1b[3mpizzazz!!\x1b[0m");
	println!("");
	println!("this is your \x1b[1mdashboard\x1b[0m. it's your one-stop pop-in-and-drop shop for tips, tricks, and commands to get you on your feet.");
	println!("");
	println!("");
	println!("\x1b[1m\x1b[35mcd\x1b[0m \x1b[2mdir\x1b[0m");
	println!("  \x1b[3mchanges the current directory\x1b[0m");
	println!("");
	println!("");
	println!("\x1b[55C\x1b[1m\x1b[35m\x1b[4Adashboard\x1b[0m");
	println!("\x1b[55C  \x1b[3malternative names: dash, home\x1b[0m");
	println!("\x1b[55C  \x1b[3mshows dashboard\x1b[0m");
	println!("");
	println!("\x1b[110C\x1b[4A\x1b[1m\x1b[35mdir\x1b[0m \x1b[2mmessage\x1b[0m");
	println!("\x1b[110C  \x1b[3mshows contents of the current directory\x1b[0m");
	println!("");
	println!("");
	println!("\x1b[1m\x1b[35mexit\x1b[0m \x1b[2mmessage\x1b[0m");
	println!("  \x1b[3malternative names: bye, goodbye, quit\x1b[0m");
	println!("  \x1b[3mexits shortcake\x1b[0m");
	println!("");
	println!("\x1b[55C\x1b[4A\x1b[1m\x1b[35mlog\x1b[0m \x1b[2mmessage\x1b[0m");
	println!("\x1b[55C  \x1b[3malternative names: echo, print, write\x1b[0m");
	println!("\x1b[55C  \x1b[3mlogs message to console\x1b[0m");
	println!("");
	println!("\x1b[110C\x1b[4A\x1b[1m\x1b[35mopen\x1b[0m \x1b[2mfile\x1b[0m");
	println!("\x1b[110C  \x1b[3malternative names: roam\x1b[0m");
	println!("\x1b[110C  \x1b[3mopen file from current directory\x1b[0m");
	println!("");
	println!("");
	println!("");
	println!("please type \x1b[1m\x1b[35mhelp\x1b[0m for all commands, questions, and more.");
	println!("");
	println!("");
	println!("");
}

#[allow(dead_code)]
fn centerline(width: u16) {
	if width >= 160 {
		for _ in 0..width/2-(160/2) {
			print!(" ");
		}
	} else {
		clear();
		checkwidth();
		std::process::exit(1);
	}
}

fn checkwidth() {

	let termsize::Size {rows, cols} = termsize::get().unwrap();

	if cols < 160 {
		clear();
		println!("terminal size: {} rows by {} columns", rows, cols);
		println!("the screen is not wide enough for shortcake to display correctly. please increase terminal width to at least 160 columns --------------------------------------+");
		std::process::exit(1);
	}
}

fn clear() {
	clearscreen::clear().expect("shortcake couldn't clear the screen");
}

fn cmdinput() {
	println!("\x1b[0m");
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let mut words: Vec<_> = input.trim().split(" ").collect();
	let command = words[0].to_lowercase();
	checkwidth();
	if command == "exit" || command == "bye" || command == "quit" || command == "goodbye" {
		words[0] = "";
		clear();

		let items = vec!["see you later", "see you later, alligator", "bye", "come again soon", "see ya", "bye bye", "toodles", "toodle-oo", "thanks for stopping by"];
		match items.choose(&mut rand::thread_rng()){
			Some(item) => print!("{}", item),
			None => print!("bye bye")
		};

		if words.join(" ").as_str().trim() == "" {
			print!(" (message: 0)");
		} else {
			print!(" (message: {})", words.join(" ").as_str().trim());
		}
		println!();
		if words.join(" ").as_str().trim() == "1" {
			std::process::exit(1);
		} else {
			std::process::exit(0);
		}
	} else if command == "log" || command == "echo" || command == "print" || command == "write" {
		words[0] = "";
		writecmdoutput(words.join(" ").as_str().trim());
	} else if command == "dash" || command == "dashboard" || command == "home" {
		dashboard();
		writecmdoutput(prettycmd().as_str());
	} else if command == "read" {
		words[0] = "";
		let file_result = &fs::read_to_string(words.join(" ").as_str().trim());
		match file_result {
			Ok(file) => {
				clear();
				let fileex = words[words.len()-1].split(".").last();
				println!("{}\n\n", color_code::color(file, fileex));
				writecmdoutput(("read file ".to_owned() + words.join(" ").as_str().trim()).as_str());
			},
		Err(error) => writecmdoutput(("failed to read ".to_owned() + words.join(" ").as_str().trim() + ": " + &interpret_error(error.to_string())).as_str())
		}
	} else if command == "abg" {
		words[0] = "";
		let file_result = &fs::read_to_string(words.join(" ").as_str().trim());
		match file_result {
			Ok(file) => {
				clear();
				abg::read(file.to_string());
			},
		Err(error) => writecmdoutput(("failed to open ".to_owned() + words.join(" ").as_str().trim() + ": " + &interpret_error(error.to_string())).as_str())
		}
	} else if command == "cd" || command == "roam" {
		words[0] = "";
		let answer = cd(words.join(" ").as_str().trim());
		writecurrentdir();
		println!("");
		println!("");
		writecmdoutput(&answer);
	} else if command == "whoami" {
		clear();
		println!("username: {}", whoami::username());
		println!("real name: {}", whoami::realname());
		println!("");
		println!("");
		writecmdoutput(prettycmd().as_str());
	} else if command == "clear" || command == "cls" {
		clear();
		writecmdoutput(prettydir().as_str());
	} else if vec!["roam..", "cd.."].contains(&input.to_lowercase().trim()) {
		words[0] = "";
		let answer = cd("..");
		writecurrentdir();
		println!("");
		println!("");
		writecmdoutput(&answer);
	} else if command == "dir" {
		writecurrentdir();
		println!("");
		println!("");
		writecmdoutput(prettycmd().as_str());
	} else if command == "cmds" || command == "cmd" {
		clear();
		println!("\x1b[1m\x1b[35mbug\x1b[0m");
		println!("  \x1b[3mreport a bug in shortcake (internet connection required)\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mcd\x1b[0m \x1b[2mdir\x1b[0m");
		println!("  \x1b[3malternative names: roam\x1b[0m");
		println!("  \x1b[3mchanges the current directory\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mclear\x1b[0m");
		println!("  \x1b[3malternative names: cls\x1b[0m");
		println!("  \x1b[3mclears screen\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mcmds\x1b[0m");
		println!("  \x1b[3malternative names: cmd\x1b[0m");
		println!("  \x1b[3mdisplays list of commands\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mdashboard\x1b[0m");
		println!("  \x1b[3malternative names: dash, home\x1b[0m");
		println!("  \x1b[3mshows dashboard\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mdir\x1b[0m \x1b[2mmessage\x1b[0m");
		println!("  \x1b[3mshows contents of the current directory\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mexit\x1b[0m \x1b[2mmessage\x1b[0m");
		println!("  \x1b[3malternative names: bye, goodbye, quit\x1b[0m");
		println!("  \x1b[3mexits shortcake\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mhelp\x1b[0m");
		println!("  \x1b[3malternative names: ?\x1b[0m");
		println!("  \x1b[3mdisplays help menu\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mlock\x1b[0m");
		println!("  \x1b[3mlocks cursor\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mlog\x1b[0m \x1b[2mmessage\x1b[0m");
		println!("  \x1b[3malternative names: echo, print, write\x1b[0m");
		println!("  \x1b[3mlogs message to console\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mopen\x1b[0m \x1b[2mfile\x1b[0m");
		println!("  \x1b[3malternative names: roam\x1b[0m");
		println!("  \x1b[3mopen file from current directory\x1b[0m");
		println!("");
		println!("\x1b[1m\x1b[35mread\x1b[0m \x1b[2mfile\x1b[0m");
		println!("  \x1b[3mopen file from current directory in plain text editor\x1b[0m");
        println!("");
        println!("\x1b[1m\x1b[35mwhoami\x1b[0m");
        println!("  \x1b[3mdisplays your username and real name\x1b[0m");
		println!("");
		println!("");
		writecmdoutput(prettycmd().as_str());
	} else if command == "help" || command == "?" {
		clear();
		println!(" * shortcake help guide *");
		println!("");
		println!("type \x1b[1m\x1b[35mcmds\x1b[0m for a list of commands.");
		println!("found a bug? type \x1b[1m\x1b[35mbug\x1b[0m to report it. (must have internet connection)");
		println!("");
		println!("");
		writecmdoutput(prettycmd().as_str());
	} else if command == "bug" {
		match open::that("https://forms.gle/B2EzcvBpaVXi8xXT7") {
			Ok(_) => writecmdoutput("opened browser without issue"),
			Err(error) => writecmdoutput(format!("there was a problem: {error}").as_str())
		}
	} else if command == "browse" {
		words[0] = "";
		let start = match words.starts_with(&["https://"]) || words.starts_with(&["http://"]) {
			true => "",
			false => "https://"
		};
		let prompt = words.join(" ");
		let trimmed_prompt = prompt.trim();
		let website = match trimmed_prompt {
			"arc" => "arc.net",
			"amazon" => "amazon.com",
			"android" => "android.com",
			"android dev" | "androiddev" => "developer.android.com",
			"bark" => "bark-coding.vercel.app",
			"bing" => "bing.com",
			"chrome" => "chrome.com",
			"coffee" | "coffeescript" | "coffee script" => "coffeescript.org",
			"discord" | ".gg" => "discord.com",
			"example" => "example.com",
			"gemini" | "google ai" | "googleai" | "google gemini" | "googlegemini" | "bard" | "google bard" | "googlebard" => "gemini.google.com",
			"gitlab" => "gitlab.com",
			"gdoc" | "gdocs" | "googledocs" | "google docs" => "docs.google.com",
			"facebook" | "fb" | "face book" => "facebook.com",
			"fileinfo" | "file info" => "fileinfo.com",
			"fonts" => "fonts.google.com",
			"gamejolt" | "game jolt" | "gj" => "gamejolt.com",
			"github" | "gh" => "github.com",
			"gmail" | "googlemail" | "google mail" => "gmail.com",
			"google" => "google.com",
			"guthib" => "guthib.com",
			"instagram" | "ig" => "instagram.com",
			"io" | "i/o" | "googleio" | "googlei/o" | "google io" | "google i/o" => "io.google",
			"jetbrains" | "jet brains" => "jetbrains.com",
			"maps" | "googlemaps" | "google maps" => "maps.google.com",
			"mdn" | "mdn web docs" | "mdnwebdocs" => "developer.mozilla.org",
			"medium" => "medium.com",
			"osdev" | "os dev" => "wiki.osdev.org",
			"phind" => "phind.com",
			"python" | "py" => "python.org",
			"qwertyy" => "linkin.bio/qwertyy__",
			"reddit" => "reddit.com",
			"rust" | "rs" => "rust-lang.org",
			"shortcake" | "shck" => "github.com/shck-ide",
			"scratch" => "scratch.mit.edu",
			"spotify" => "spotify.com",
			"stackoverflow" | "stack overflow" | "so" => "stackoverflow.com",
			"techcrunch" => "techcrunch.com",
			"twitter" | "x" => "x.com",
			"universe" | "github universe" | "githubuniverse" | "ghuniverse" | "gh universe" => "githubuniverse.com",
			"vercel" => "vercel.com",
			"verge" | "the verge" | "theverge" => "theverge.com",
			"vscode" | "visualstudiocode" | "visual studio code" => "vscode.dev",
			"whitepages" | "white pages" => "whitepages.com",
			"wikipedia" => "wikipedia.org",
			"wordle" => "nytimes.com/wordle",
			"youtube" | "yt" => "youtube.com",
			_ => trimmed_prompt
		};
		match open::that(format!("{}{}", start, website.trim())) {
			Ok(_) => writecmdoutput("opened browser without issue"),
			Err(error) => writecmdoutput(format!("there was a problem: {error}").as_str())
		};
	} else if command == "lock" {
		loop {
			print!("\x1b[H");
		}
	} else if command == "mkdir" {
		words[0] = "";
		let joined = words.join(" ");
		let joined_trimmed = joined.trim();
		let path = std::path::Path::new(&joined_trimmed);
		match fs::create_dir_all(path) {
			Ok(_) => {
				writecurrentdir();
				println!("");
				println!("");
				writecmdoutput("successfully created directory");
			},
			Err(error) => {
				writecurrentdir();
				println!("");
				println!("");
				writecmdoutput(format!("there was an error: {}", interpret_error(error.to_string())).as_str());
			}
		};
	} else if command == "rmdir" {
		words[0] = "";
		let joined = words.join(" ");
		let joined_trimmed = joined.trim();
		let getdir = getdir();
		let dir = match joined_trimmed {
			"" | "." => getdir.as_str(),
			&_ => joined_trimmed
		};
		if joined_trimmed == "" || joined_trimmed == "." {
			cd("..");
		};
		let path = std::path::Path::new(&dir);
		match fs::remove_dir(path) {
			Ok(_) => {
				writecurrentdir();
				println!("");
				println!("");
				writecmdoutput("successfully deleted directory");
			},
			Err(error) => {
				writecurrentdir();
				println!("");
				println!("");
				writecmdoutput(format!("there was an error: {}", interpret_error(error.to_string())).as_str());
			}
		};
	} else if command == "rmdir!" {
		words[0] = "";
		let joined = words.join(" ");
		let joined_trimmed = joined.trim();
		let getdir = getdir();
		let dir = match joined_trimmed {
			"" | "." => getdir.as_str(),
			&_ => joined_trimmed
		};
		if joined_trimmed == "" || joined_trimmed == "." {
			cd("..");
		};
		match fs::remove_dir_all(dir) {
			Ok(_) => {
				writecurrentdir();
				println!("");
				println!("");
				writecmdoutput("successfully deleted directory + contents of directory");
			},
			Err(error) => {
				writecurrentdir();
				println!("");
				println!("");
				writecmdoutput(format!("there was an error: {}", interpret_error(error.to_string())).as_str());
			}
		};
	}

	
	else if command == "" {
		writecmdoutput(prettycmd().as_str());
	}

    // easter eggs
	else if command == "hello" || command == "hi" || command == "hello," {
		writecmdoutput("hello!! :)");
	} else if input.to_lowercase().trim() == "i hate tapeworms" {
		writecmdoutput("thank you so much");
	} else if command == "pizzazz" {
		for _ in 0..1000 {
			thread::sleep(time::Duration::from_millis(1));
			print!("\x1b[H");
			let mut rng = rand::thread_rng();
			for _ in 0..rng.gen_range(0..35) {
				print!("\x1b[1B");
			}
			for _ in 0..rng.gen_range(0..160) {
				print!("\x1b[1C");
			}
			print!("✨");
		}
	}

	else {
		writecmdoutput(("no such command \"".to_owned() + &command + "\"\x1b[0m").as_str());
	}
}

fn getdir() -> String {
	match current_dir() {
		Ok(dir) => return dir.display().to_string(),
		Err(error) => return "failed to get current directory: ".to_owned() + &interpret_error(error.to_string())
	}
}

fn getdircontentnum() -> usize {
	match fs::read_dir(getdir()) {
		Ok(entries) => return entries.count(),
		Err(error) => {
			writecmdoutput(("error: ".to_owned() + &interpret_error(error.to_string())).as_str());
			return 0;
		}
	}
}

fn prettydir() -> String {
	return getdir().replacen(("/home/".to_owned() + whoami::username().as_str()).as_str(), "~", 1) + "/";
}

fn scrollline(message: &str) {
	let mut unimportant = String::new();
	let _ = io::stdin().read_line(&mut unimportant);
	println!("\x1b[1A\x1b[K{}", message);
}

/*
fn writecmdoutput(message: &str) {
	// \x1b[H\x1b[35B\x1b[2K
	print!("\x1b[0m\x1b[40m\x1b[37m");
	let mut i: usize = 0;
	let chars: Vec<_> = message.chars().collect();
	while i < 160 {
		if i >= chars.to_owned().len() {
			print!(" ");
		} else {
			print!("{}", chars[i]);
		}
		i += 1;
	}
	let batteryp = battery::Manager::new().unwrap().batteries().unwrap().next().unwrap().unwrap().state_of_charge().get::<battery::units::ratio::percent>().round();
	print!("\x1b[132G\x1b[42m\x1b[30m 🕒 {}:{} {}/{}/{} ", Local::now().format("%0H").to_string(), Local::now().format("%0M").to_string(), Local::now().format("%m").to_string(), Local::now().format("%d").to_string(), Local::now().year());
	if batteryp == 100.00 {
		print!("\x1b[42m 🔋{}% ", batteryp.to_string().replace(".0", ""));
	} else if batteryp > 60.0 {
		print!("\x1b[42m 🔋 {}% ", batteryp.to_string().replace(".0", ""));
	} else if batteryp > 20.0 {
		print!("\x1b[43m 🔋 {}% ", batteryp.to_string().replace(".0", ""));
	} else if batteryp > 10.0 {
		print!("\x1b[41m 🪫 {}% ", batteryp.to_string().replace(".0", ""));
	} else {
		print!("\x1b[41m 🪫 {}%  ", batteryp.to_string().replace(".0", ""));
	}
	print!("\x1b[0m\x1b[A\x1b[2K\x1b[A");
}
*/

fn writecmdoutput(message: &str) {
	let batteryp = battery::Manager::new().unwrap().batteries().unwrap().next().unwrap().unwrap().state_of_charge().get::<battery::units::ratio::percent>().round();
	print!("\x1b[2K\x1b[0m\x1b[35m  1.0.0 (lazy edition)  \x1b[0m");
	if batteryp > 60.0 {
		print!("\x1b[32m\x1b[1m  {}%  ", batteryp.to_string().replace(".0", ""));
	} else if batteryp > 25.0 {
		print!("\x1b[33m  {}%  ", batteryp.to_string().replace(".0", ""));
	} else if batteryp > 10.0 {
		print!("\x1b[31m\x1b[1m  {}%  ", batteryp.to_string().replace(".0", ""));
	} else {
		print!("\x1b[31m\x1b[5m\x1b[1m  {}%  ", batteryp.to_string().replace(".0", ""));
	}
	print!("\x1b[0m\x1b[36m  {}\x1b[5m:\x1b[0m\x1b[36m{} {}/{}/{}  \x1b[0m  {}\x1b[A\x1b[2K\x1b[A", Local::now().format("%0H").to_string(), Local::now().format("%0M").to_string(), Local::now().format("%m").to_string(), Local::now().format("%d").to_string(), Local::now().year(), message);
}

fn writecurrentdir() {
	match fs::read_dir(getdir()) {
		Ok(entries) => {
			clear();
			println!("\x1b[Hshowing contents of {}/ ({} item{})", getdir(), getdircontentnum(), match getdircontentnum() { 1 => "", _ => "s" });
			println!("\x1b[1G\x1b[7C\x1b[1m\x1b[36mDIR\x1b[1G\x1b[17C\x1b[0m|  \x1b[36m.\x1b[0m");
			println!("\x1b[1G\x1b[7C\x1b[1m\x1b[36mDIR\x1b[1G\x1b[17C\x1b[0m|  \x1b[36m..\x1b[0m");
			let mut i = 0;
			for entry in entries {
				i += 1;
				match entry {
					Ok(entry) => {
						match entry.file_type() {
							Ok(filetype) => {
								let hidden = match entry.file_name().as_os_str().as_bytes()[0] {
									b'.' => "\x1b[2m\x1b[3m",
									_ => "\x1b[0m"
								};
								let space = match hidden {
									"\x1b[0m" => String::from("\x1b[2m•\x1b[0m"),
									_ => format!("\x1b[0m•{}", hidden)
								};
								if filetype.is_dir() {
									println!("{}", (format!("{}{}", hidden, i) + "\x1b[1G\x1b[7C\x1b[1m\x1b[36mDIR\x1b[1G\x1b[17C\x1b[0m|  " + hidden + "\x1b[36m" + entry.path().to_str().unwrap().replace((getdir()+"/").as_str(), "").replace(" ", "\x1b[2m•\x1b[0m\x1b[36m").as_str() + "/\x1b[0m").as_str());
								} else {
									let typename = file_desc::get_file_desc(entry.path().to_str().unwrap().replace((getdir()+"/").as_str(), ""));
									println!("{}", (format!("{}{}", hidden, i) + "\x1b[1G\x1b[7C\x1b[1m\x1b[34m" + &typename + "\x1b[1G\x1b[17C\x1b[0m|  " + hidden + entry.path().to_str().unwrap().replace((getdir()+"/").as_str(), "").replace(" ", &space).as_str() + "\x1b[0m").as_str())
								}
							},
							Err(error) => println!("{}", ("error: ".to_owned() + &interpret_error(error.to_string())).as_str())
						}
					},
					Err(error) => println!("{}", ("error: ".to_owned() + &interpret_error(error.to_string())).as_str())
				}
			}
		},
		Err(error) => writecmdoutput(("error: ".to_owned() + &interpret_error(error.to_string())).as_str())
	}
}

fn cd(path: &str) -> String {
	return match set_current_dir(path) {
		Ok(_) => prettycmd(),
		Err(error) => "failed to change current directory: ".to_owned() + &interpret_error(error.to_string())
	}
}

fn interpret_error(error: String) -> String {
	let binding = error.to_lowercase();
	return String::from(match error.as_str() {
		"Operation not permitted (os error 1)" => "operation isn't permitted (os error 1)",
		"No such file or directory (os error 2)" => "the file or directory doesn't exist (os error 2)",
		"The system cannot find the path specified. (os error 3)" => "path not found (os error 3)",
		"Interrupted system call (os error 4)" => "interrupted system call (os error 4)",
		"Access is denied. (os error 5)" => "access denied (os error 5)",
		"No such device or address (os error 6)" => "device or address doesn't exist (os error 6)",
		"Exec format error (os error 8)" => "exec format error (os error 8)",
		"Bad file descriptor (os error 9)" => "bad file descriptor (os error 9)",
		"Resource temporarily unavailable (os error 11)" => "resource is temporarily unavailable (os error 11)",
		"Permission denied (os error 13)" => "permission denied (os error 13)",
		"Invalid cross-device link (os error 18)" => "invalid cross-device link (os error 18)",
		"No such device (os error 19)" => "device does not exist (os error 19)",
		"Not a directory (os error 20)" => "not a directory (os error 20)",
		"Is a directory (os error 21)" => "is a directory (os error 21)",
		"Invalid argument (os error 22)" => "argument is invalid (os error 22)",
		"Too many open files (os error 24)" => "there are too many files open (os error 24)",
		"No space left on device (os error 28)" => "no space left on this device (os error 28)",
		"Read-only file system (os error 30)" => "file system is read-only (os error 30)",
		"The process cannot access the file because it is being used by another process. (os error 32)" => "the file is in use by another process (os error 32)",
		"Directory not empty (os error 39)" => "the directory is not empty (os error 39) to force, use rmdir! instead (this cannot be undone)",
		"stream did not contain valid UTF-8" => "file not utf-8 encoded",
		_ => binding.as_str()
	});
}

fn prettycmd() -> String {
	return whoami::username() + "@" + &fallible::hostname().expect("failed to get host name") + ":" + &prettydir();
}