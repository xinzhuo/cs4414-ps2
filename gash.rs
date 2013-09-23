extern mod extra;

use std::{io, run, os, path, task};
use extra::{deque};


fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
	let HOME = os::homedir();
	let mut history: @mut extra::deque::Deque<~[~str]> = @mut extra::deque::Deque::new();

	println(fmt!("%?", HOME));
    
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

	match program {
                ~"exit"     => {return; }
				~"cd"		=> { if !argv.is_empty() {cdpre(argv.remove(0)); }
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
									let temp = argv;
									task::spawn_sched(task::SingleThreaded, | | {
										run::process_status(program, temp);
									});
								}
								else{run::process_status(program, argv);}
								}
            }

}

//CD Command
fn cdpre(s: ~str){
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
	let isDir:bool = os::path_is_dir(p);
	if exists && isDir{
		os::change_dir(p);
	} else if exists{
		println(fmt!("gash: cd: %s: Not a directory", p.to_str()));
	} else {
		println(fmt!("gash: cd: %s: No such file or directory", p.to_str()));
	}
}

