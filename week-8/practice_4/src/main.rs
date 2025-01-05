fn main() {
    
    // name vector
    let name = vec!["mary","sam","sally","greg","ade","mark","june","ife"];

    // Age vector 
    let age = vec![16,17,119,22,20,21,18,23];

    print!("\nAge allocation:\n");

    //loop to itrate elements in vector
    for i in 0..age.len()
    {
    	//iterating through i ont he vector
    	print!("{} is {} years old\n", name[i],age[i]);

    }

}

