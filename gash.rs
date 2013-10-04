extern mod extra;

use std::{io, run, os, path, task, int, str, libc};
//use extra::{deque};

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
        let argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
			run_program(argv.to_owned(), line.ends_with("&"), history);            
			history.add_front(argv);
        }
    }
}

fn run_program(args: ~[~str], run_in_background: bool, history: @mut extra::deque::Deque<~[~str]>) {
	let mut argv = args;
	let mut current_argv = ~[];
	let mut argv_modes = ~[];
	let mut pipe = false;
	let mut pipe_program = ~"";

	//Determine role of each parameter
	if argv.len() > 0 {
		let program = argv.remove(0);
		unsafe { while argv.len() > 0 {
			let sig = argv.unsafe_get(0);
			match sig {
				~"|"	=>	{
								//println("Found a pipe");
								argv.remove(0);
								pipe=true;
								break; 
							}
				~">"	=>	{
								//println("Found redirect output");
								argv.remove(0); 
								argv_modes.push(OUTPUT_REDIRECTION); 
								if argv.len() > 0 { current_argv.push(argv.remove(0)); }
								else{println("Syntax Error"); return;}
							}
				~"<"	=>	{
								//println("Found redirect input");
								argv.remove(0); 
								argv_modes.push(INPUT_REDIRECTION);
								if argv.len() > 0 { current_argv.push(argv.remove(0));}
								else{println("Syntax Error"); return;}
							}
				_		=>	{
								current_argv.push(argv.remove(0));
								argv_modes.push(NORMAL); 
							}
			}
				
	 	} }
	//println(fmt!("argv: %? \n current_argv: %? \n modes: %?", argv, current_argv, argv_modes));			
		assert!(current_argv.len() == argv_modes.len());
		
		//Get Read and Write files
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
			~"exit"		=>	{ return; }

			~"cd"		=>	{ 
								if !current_argv.is_empty() {
									pre_cd(current_argv.remove(0)); 
								}
								else { 
									cd(~os::getcwd())
								} 
							}

			~"add"		=>	{ 
								if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)).get() 
										+ int::from_str(current_argv.remove(0)).get();
									println(fmt!("%i", result));
								}
							}
			
			~"sub"		=>	{
								if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)).get() 
										- int::from_str(current_argv.remove(0)).get();
									println(fmt!("%i",result));							
								} 
							}

			~"mul"		=>	{
								if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)).get() 
										* int::from_str(current_argv.remove(0)).get();
									println(fmt!("%i",result));							
								} 
							}

			~"div"		=>	{
								if !current_argv.len() >= 2 {
									let result = int::from_str(current_argv.remove(0)).get() 
										- int::from_str(current_argv.remove(0)).get();
									println(fmt!("%i",result));							
								} 
							}

			~"search"	=>	{
								let matcher = current_argv.remove(0);
								let mut found = false;
								for history.rev_iter().advance|s| {
									for s.iter().advance |word| {
										if word.contains(matcher) {
											for s.iter().advance |pword| {print(*pword + " ") }
											found = true;
											println("");
										}									
									}
								}
								if !found {
									println("No Match Found");
								}
							}

			~"history"	=>	{
								let mut i = 1;
								for history.rev_iter().advance |s| {
									print(fmt!("%i ",i));
									for s.iter().advance |word| {
										print(*word + " ") 
									}
							 		println("");
									i+=1;
						   	 	}	
							}		
			_           =>	{					
								let temp = current_argv;
								let r = readfile;
								let w = writefile;		 
								let mut process_options = ~run::ProcessOptions::new();
								//Default stdin, stdout, stderr
								process_options.in_fd = Some(0);
								process_options.out_fd = Some(1);								
								process_options.err_fd = Some(2);
									
								unsafe {
									if w != ~"" {
										let fp = std::libc::fopen(
											w.as_c_str(|x| x),
											"w".as_c_str(|x| x));
										process_options.out_fd = Some(libc::fileno(fp));
									}
									if r != ~"" {	
										if os::path_exists(~path::Path(r)) {
											let fp = std::libc::fopen(
												r.as_c_str(|x| x),
												"r".as_c_str(|x| x));
											process_options.in_fd = Some(libc::fileno(fp));
										}
									}
									
								}
								//println(fmt!("%?", process_options));						
								
								if run_in_background {
									task::spawn_sched(task::SingleThreaded, | | {
										let mut process_options = ~run::ProcessOptions::new();
										process_options.in_fd = Some(0);
										process_options.out_fd = Some(1);
										process_options.err_fd = Some(2);						
										unsafe {
											if w != ~"" {
												let fp = std::libc::fopen(
													w.as_c_str(|x| x),
													"w".as_c_str(|x| x));
												process_options.out_fd = Some(libc::fileno(fp));
											}
											if r != ~"" {	
												if os::path_exists(~path::Path(r)) {
													let fp = std::libc::fopen(
														r.as_c_str(|x| x),
														"r".as_c_str(|x| x));
													process_options.in_fd = Some(libc::fileno(fp));
												}
											}
										}
										run::Process::new(program, temp, *process_options);
									});
								}								
								else { 
									run::Process::new(program, temp, *process_options);
								} 
							}
		}
		if pipe {
			run_program(argv, run_in_background, history);
		}

	}
}

fn pre_cd(s: ~str){
    if s.starts_with("~"){ cd(~path::Path(s.slice(1, s.len())));  }
    else{ cd(~path::Path(s)); }
}

fn cd(p: &Path) {
	let exists:bool = os::path_exists(p);
	let is_dir:bool = os::path_is_dir(p);
	if exists && is_dir{ os::change_dir(p); }
	else if exists{ println(fmt!("gash: cd: %s: Not a directory", p.to_str()));	} 
	else { println(fmt!("gash: cd: %s: No such file or directory", p.to_str())); }
}

