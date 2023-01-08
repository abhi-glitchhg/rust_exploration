fn main(){
    let x = 1;
    println!("{}", x);

    //it will error as we cant mutate x; hence commenting;
    // x =1;
    // redefining stuff is allowed, and in this process data type can change. 
    let x :u32 = 32;
    println!("{}",x);

    let g:bool = false;


    if g
    {println!("this is from inside if block");
    }
    else {
    println!("this is from false blocl");
    }

    let mut gg = 10;
    println!("{}", gg);
    gg= 15;
    println!( " updated value\n{}", gg);

    // tuple

    let lmao = ("abhijit", 10.45, false);
    println!("{}", lmao.0 );
    // any data type is not mutable by default. so if we want to change then lamo should be mutable
    // variable.
    //lmao.0 = "abhijit deo";
    let mut lmao = ("abhijit", 10.45, false);
    lmao.0 = "abhijit deo";

    println!("{}", lmao.0 );
    //println!("{}", lmao[0]); // does this work? -> nope gives error, we can only do tuple.index not
                             // tuple[index];
    // okay so i kind of played with the 
    

}
