mod commonregex;



 
 

    
    
fn main() {
    let text = "John, please get that article on www.linkedin.com to me by 5:00PM 
                               on Jan 9th 2012. 4:00 would be ideal, actually. If you have any 
                               questions, You can reach me at (519)-236-2723x341 or get in touch with
                               my associate at harold.smith@gmail.com";
    let caps = commonregex::CommonRegex(text);
                            
    println!("{:?}", caps);
    println!("{:?}",commonregex::times("When are you free? Do you want to meet up for coffee at 4:00?"));
    //prints ["4:00"]
    println!("{:?}",commonregex::prices("They said the price was US$5,000.90, actually it is US$3,900.5. It\'s $1100.4 less, can you imagine this?"));
    //prints ["US$5,000.90", "US$3,900.5", "$1100.4"]
    println!("{:?}",commonregex::ipv6s("The IPv6 address for localhost is 0:0:0:0:0:0:0:1, or alternatively, ::1."));
    //prints ["0:0:0:0:0:0:0:1", "::1"]
}
