fn main(){
    ownership_transfer_with_functions();

}


fn ownership_transfer_with_functions(){

    let s1 = String::from("Hello");
    println!("{} from ownership transfer with function's main scope", s1);
    no_ops_function(s1);
    // next statement will give compilation error; Reason -> 
    // when we call the no_ops_function, the ownership of the s1 goes to the input_string variable
    // and this variable/data is dropped by rust memory management system when we go out of the
    // scope of the function. 
    

    // println!("{} from ownership transfer with function's main scope", s1);

    // now lets see how we can avoid this and pass the string as a referance and return the same
    // referance.
    
    let s1 = String::from("Hello");
    let s2 = returns_same_string(&s1); // remember we need to pass the referenace .

    // now try to print s1 and s2; 
    println!("{} is s1 and {} is s2",s1, s2); // now we have same reference in s1 and s2.
    // can we also check if these two are equal or not; (my bad they are different types; one is
    // string and another is reference to the string);
    //in my opionion they should be.
    let boolean_val = &s1==s2; // 
    println!("{} = comparison of s1 and s2", boolean_val );


    // hmm so the above s1 was not mutable; and i think it was was easy to work with unmutable data
    // ---------------------------------------------------------------------------------------------------------------------------------------//


    let mut s1 = String::from("hello");

    println!("{} is a mutable strinf from ownership transfer function.", s1);

}

fn no_ops_function(input_string:String){

    println!("{} from no-ops-function", input_string);

}

fn returns_same_string(input_string:&String) ->&String {
    println!("{} returns same string function", input_string);
    &input_string
}
