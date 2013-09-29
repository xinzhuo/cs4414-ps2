all: gash

gash: gash.rs
	rustc gash.rs


clean: 
	rm -rf *~ 
