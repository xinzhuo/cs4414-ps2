use std::{io, run, os, path};

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
	let HOME = os::homedir();
	println(fmt!("%?", HOME));
    
    loop {
        print(os::getcwd().to_str() + ": " + CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
				~"cd"		=> { if !argv.is_empty() {cdpre(argv.remove(0)); }
								 else { cd(~os::getcwd())} 
							   }
                _           => {run::process_status(program, argv);}
            }
        }
    }
}
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
