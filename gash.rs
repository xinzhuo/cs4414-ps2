extern mod extra;

use std::{io, run, os, path, task};
use extra::{deque};

static NORMAL:int = 0;
static OUTPUT_REDIRECTION:int = 1;
static OUTPUT_REDIRECTION_APPEND:int = 2;
static INPUT_REDIRECTION:int = 3;
static PIPELINE:int = 4; 

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
	let history: @mut extra::deque::Deque<~[~str]> = @mut extra::deque::Deque::new();
    
    loop {
        print(os::getcwd().to_str() + ": " + CMD_PROMPT);
        let line = io::stdin().read_line();
		
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
			history.add_front(argv.to_owned());
            let program = argv.remove(0);
			run_program(program, argv, line.ends_with("&"), history);            
        }
    }
}

fn run_program(program: ~str, args: ~[~str], run_in_background: bool, history: @mut extra::deque::Deque<~[~str]>) {

	let mut argv = args;
	let mut current_argv = ~[];
	let mut argv_modes = ~[];

	unsafe {while argv.len() > 0 {
		let temp2 = argv.unsafe_get(0);
		match temp2 {
			~"|"		=> {println("Found a pipe");
							argv.remove(0);
							argv_modes.push(PIPELINE);
							break; }
			~">"		=> {println("Found redirect output");
							argv.remove(0); 
							argv_modes.push(OUTPUT_REDIRECTION); }
			~">>"		=> {println("Found redirect output (append)");
							argv.remove(0); 
							argv_modes.push(OUTPUT_REDIRECTION_APPEND); }
			~"<"		=> {println("Found redirect input");
							argv.remove(0); 
							argv_modes.push(INPUT_REDIRECTION);}
			_			=> {current_argv.push(argv.remove(0));
							argv_modes.push(NORMAL); }
		}
		println(fmt!("%? \n %?", argv, current_argv));		
 	} }


	match program {
                ~"exit"     => {return; }
				~"cd"		=> { if !current_argv.is_empty() {pre_cd(current_argv.remove(0)); }
								 else { cd(~os::getcwd())} 
							   }
				~"history"	=> { let mut i = 1;
								 for history.rev_iter().advance |s| {
									print(fmt!("%i ",i));
									for s.iter().advance |word| {print(*word + " ") }
								 	println("");
									i+=1;
							   	 }
								}
                _           => {if run_in_background {
									let temp = current_argv;
									task::spawn_sched(task::SingleThreaded, | | {
										run::process_status(program, temp);
									});
								}
								else{run::process_status(program, current_argv);}
								}
            }
	if argv.len() > 0 {
		run_program(argv.remove(0), argv, run_in_background, history);
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
