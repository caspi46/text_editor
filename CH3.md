Chapter 3: Raw Input and Output

Getting to Know Idiomatic Code 

if let : for the case where there is only one branch in match 

Clippy: 

Improved Error Handling: 
- Handle locally: If we can address an error where it occurs, we'll take care of it 
- Pass upwards: for errors we can't fix directly, pass them up the chain for further handling 
- Top-level handling: If an error reaches the highest level, we'll manage it as gracefully as possible 

Press Ctrl-Q to quit 
- Current version: just press q => quit 
- now need to hit Ctrl-Q to quit, so can press 'q' as a character, 'q' 

Clear the screen:
- Starting up: setting the stage for the user 
- After each keypress: To respond to the user's actions
- Before existing: To clean up our workspace and leave a tidy farewelll message 

print!("\x1b[2J"); 
- \x: what follows should be read as a hexadecimal # 
- 1b: translates to 27 in decimal 

- \x1b[1J = clear up to the cursor 
- \x1b[0J = clears from the cursor to the end of the screen 
- Simply \x1b[J defaults to clearing from the cursor to the end 


Self::initialize().unwrap();
let result = self.repl(); 
Self::terminate().unwrap();
result.unwrap(); 

3 places where can run into an error: 
- During the preparation of the terminal 
- During execution of our loop 
- During shutting down our terminal 

Simply unwrap since nothing we can do to handle this  => no need to do own panicing 

Assignment 1: Tildes 


Assignment 2: Improve it 

Ownership 
- owner-borrower relationship 

Trait