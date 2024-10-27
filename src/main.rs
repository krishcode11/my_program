fn main() {
    let num:u8 = 5;
    println!("this is stored in num {} ", num);

    // string
    let string_literal =String::from("Hi,KRISHAN!!!");
    println!("this is string literal  {}", string_literal);

    //Trupple
    let emp_info:(&str,u8)  = ("Alex", 30);
    
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    //destructuring
    let(employee_name,employee_age) = emp_info;
    println!("Employe Name={}, Employe Age={}", employee_name,employee_age);
    println!("Employe Name={}, Employe Age={}", emp_name,emp_age);

    let a = 5;
    let b = a;
    println!("a={}",a);
    println!("b={}",b);
     
    let str1 = String::from("Hello");   // heap memory management = ownership rule
    let str2= str1.clone();  //transfer of ownership === rule 2 . value its ownership
    println!("str1={}",str1);
    println!("str2={}",str2);

    let s1:String = get_string();
    println!("this is s1:{}",s1); //s1 is the owner of hello

    let s2:String = String::from("world"); //s2 is the owner of world
    let s3:String = send_get_string(s2); //transfer of ownership from s2 to received_string
    println!("this is s3:{}",s3); //s3 is the new owner of worrld


    let s1:String = String::from("hello"); //s1 is owner
    let (s2,len) = calculate_length(s1);  // transfer ownership
    println!("the length of {} is {}" ,s2,len); 
}

fn calculate_length(s:String)->(String,usize){
let length:usize = s.len();
return (s,length);
}   //s will be new owner
     

fn get_string()->String{
    let new_string = String::from("hello");
    return new_string; //transfering the ownership
}

fn send_get_string(received_string:String)->String{
    return received_string; //transfer of ownership from //received_string to s3
}
//fn process_string(item:String){
  //  println!("the value of x in process_string() is {}",item);
//}


//fn process_integer(x:u8){
 //   println!("the value of x is process_integer() is {}",x);
//}


//fn add(item1:u8,item2:u8)->u8{
 //   return item1+item2;
//}
//signed integer = (-2)^n-1 to 2^n-1 - 1
//unsigned integer = 0 to 2^n  - 1
//String - Dynamic Length Strings - Heap Allocated 
//&str - Fixed Length Strings - Special Memory

//fn print_value(item:u8){
    //println!("{}",item);
//}
    