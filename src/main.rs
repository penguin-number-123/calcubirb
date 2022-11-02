use std::io::{stdin,stdout,Write};
mod arbitary;
fn main() {
    println!("CAScubirb CLI Interface - V0.0.0 ");
    let mut count:i32 = 0;
    loop{
        let mut s=String::new();
        print!("In[{}]:= ",count);
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "exit" || s=="Exit"{
            println!("Out[{}]: Ending session...",count);
            std::process::exit(0);
        }else if s=="birb birb"{
            println!("Out[{}]: Birb birb (^)>",count);
            std::process::exit(0);
        }
        else{
            println!("Out[{}]: {}",count,s);
        }
        
        count += 1;
        
    }

}
