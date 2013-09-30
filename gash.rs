extern mod extra;

use std::{io, run, os, path, task, int, str};
//use extra::{deque};

static NORMAL:int = 0;
static OUTPUT_REDIRECTION:int = 1;
static INPUT_REDIRECTION:int = 3;
static PIPELINE:int = 4; 

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
	let history: @mut extra::deque::Deque<~[~str]> = @mut extra::deque::Deque::new();
    
    loop {
        print(os::getcwd().to_str() + ": " + CMD_PROMPT);
        let line = io::stdin().read_line();
		
        debug!(fmt!("line: %?", line));
        let argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
			history.add_front(argv.to_owned());
			run_program(argv, line.ends_with("&"), history);            
        }
    }
}

fn run_program(args: ~[~str], run_in_background: bool, history: @mut extra::deque::Deque<~[~str]>) {
	let mut argv = args;
	let mut current_argv = ~[];
	let mut argv_modes = ~[];
	let mut pipe = false;

	if argv.len() > 0 {
		let program = argv.remove(0);
		unsafe {while argv.len() > 0 {
			let sig = argv.unsafe_get(0);
			match sig {
				~"|"		=> {println("Found a pipe");
								argv.remove(0);
								pipe=true;
								break; }
				~">"		=> {println("Found redirect output");
								argv.remove(0); 
								argv_modes.push(OUTPUT_REDIRECTION); 
								if argv.len() > 0 { current_argv.push(argv.remove(0)); }
								else{println("Syntax Error"); return;}
								}
				~"<"		=> {println("Found redirect input");
								argv.remove(0); 
								argv_modes.push(INPUT_REDIRECTION);
								if argv.len() > 0 { current_argv.push(argv.remove(0));}
								else{println("Syntax Error"); return;}
								}
				_			=> {current_argv.push(argv.remove(0));
								argv_modes.push(NORMAL); }
			}
			println(fmt!("%? \n %?", argv, current_argv));		
	 	} }
		assert!(current_argv.len() == argv_modes.len());
		let mut i= 0;
		let mut writefile = ~"";
		let mut readfile = ~"";
		while i < argv_modes.len() {
			if argv_modes[i] == OUTPUT_REDIRECTION {
				writefile = current_argv.remove(i);
				argv_modes.remove(i);
			}
			else if argv_modes[i] == INPUT_REDIRECTION {
				readfile = current_argv.remove(i);
				argv_modes.remove(i);
			}
			else { i+=1; }
		}
		
		match program {
                ~"exit"     => {return; }
				~"cd"		=> { if !current_argv.is_empty() {pre_cd(current_argv.remove(0)); }
								 else { cd(~os::getcwd())} 
							   }
				~"add"		=> { if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)) + int::from_str(current_argv.remove(0));
									match result {
										Some(ref x) => println(fmt!("%i",*x)),
										None => ()
									}
								 } 
								}
				~"sub"		=> { if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)).get() - int::from_str(current_argv.remove(0)).get();
									println(fmt!("%i",result));
									
								 } 
								}
				~"mul"		=> { if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)).get() * int::from_str(current_argv.remove(0)).get();
									println(fmt!("%i",result));
								 } 
								}
				~"div"		=> { if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)).get() / int::from_str(current_argv.remove(0)).get();
									println(fmt!("%i",result));
								 } 
								}
				~"history"	=> { let mut i = 1;
								 for history.rev_iter().advance |s| {
									print(fmt!("%i ",i));
									for s.iter().advance |word| {print(*word + " ") }
								 	println("");
									i+=1;
							   	 }
								}
                _           => {								 
								if run_in_background {
									let temp = current_argv;
									
									if writefile == ~"" {
									task::spawn_sched(task::SingleThreaded, | | {
										run::process_status(program, temp);
									}); }
									else {
									let w = writefile;
									task::spawn_sched(task::SingleThreaded, | | {
										let write_result = io::buffered_file_writer(~path::Path(w));
										if write_result.is_ok() {

											let file = write_result.unwrap();
											let printout = run::process_output(program, temp);
											file.write(printout.output);
										} } ); 	
									}
								}
								else{ 
										if readfile == ~"" {
											//let input_reader = run::input();
										}
										if writefile == ~"" {let printout = run::process_output(program, current_argv); println(str::from_bytes(printout.output));} 
										else {
										let write_result = io::buffered_file_writer(~path::Path(writefile));
										if write_result.is_ok() {
											let file = write_result.unwrap();
											let printout = run::process_output(program, current_argv);								if !pipe {
											file.write(printout.output); }
											else { argv.insert(1, printout.output.to_str());}
										}	
									}
								} }
            }
		if pipe {
			run_program(argv, run_in_background, history);
		}

	}
}

fn pre_cd(s: ~str){
    if s.starts_with("~"){
        println("Go");
        cd(~path::Path(s.slice(1, s.len()))); 
    }
    else{
        cd(~path::Path(s)); 
    }
}

fn cd(p: &Path) {
	let exists:bool = os::path_exists(p);
	let is_dir:bool = os::path_is_dir(p);
	if exists && is_dir{
		os::change_dir(p);
	} else if exists{
		println(fmt!("gash: cd: %s: Not a directory", p.to_str()));
	} else {
		println(fmt!("gash: cd: %s: No such file or directory", p.to_str()));
	}
}

fn load(filename: ~str) -> ~[~str] {
	let read_result = io::file_reader(~path::Path(filename));
	if read_result.is_ok() {
		let file = read_result.unwrap();
		return file.read_lines();
	}
	println(fmt!("Error reading file: %?", read_result.unwrap_err()));
	return ~[];
}
