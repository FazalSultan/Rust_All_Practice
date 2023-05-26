// use std::slice::range;

// use std::io;
fn main()
//_______________________Important Notes/ Comments_______________________

/*
_________________________STRING TYPES _____________________________
-- &str ----->> Fixed is size and store on Stack
-- String --->> Can be grow  and store on Heap



cargo run is using for compliling and running a rust Program.
The print is used for displaying something on the screen but the (ln --> is to show something on new line) with print -->(println!("Abc"))
 */
{
    // println!("Hello World");
    // let s_1 =String::from("Fazal");
    // println!("The string is: {}",s_1);

    // let s_2 =String::from("Sultan");
    // let s_3 =s_1+ &s_2; --------------------------->> &s_2 means in case of Non permitive a move occure and the ownership Changed therefore.
    // println!("The First Name and Last Name is : {}",s_3);




    //____________________________Concept of Shadowing____________________________________________

    /*
    def : The new updated value will be consider by the compiler, if the variables are with the same Names.

    With the "Let keyword" we declare a variable in Rust
    .But inside the scope when we intalize the same variable with the let keyword the value will be update inside the code segmnent but without the let inside
    the segment the vlaue will be Parmently changed.
     */

    // let mut p=13;
    // println!("Outside the Code segment :{}",p);
    // {
    //     p= 12;
    //     println!("Inside the Code segment :{}",p);
    // }
    // println!("Outside the Code segment :{}",p);






    //____________________________________CONSTANTS /RULES...!!_______________________________________
    /*
           -->> We cant use the "Mut" with Constant.
           -->> The User will be Responsible for The type of the Contant means manually we will write the Type of the Variable.
           -->> Rust convention For Declaring a constant .i,e(const MAX_SALARY), all are capital and separated with underscore(_).

    */
    // const MAX_SALARY:u32 =4521;
    // println!("The Constant is : {}",MAX_SALARY);




    // ________________________________________TUPLES___________________________________________________
    /*
       --> Tuples are immutable ----->>(Not Changable)
       --> For accessing we need . and varibale name
       --> we cant find the length of a tuples bcoz in it contain d/f types of values.

    */

    // let tp =("agile" ,"Name", "Sultan","DoB");
    // let tp1 =(1,2,4,5,0,1);
    // let var_length =tp.0.len();
    // println!("My Tuples is: {} {} {}",tp.2,tp.0,tp.4);
    // println!("My Tuples is: {:?}",tp);
    // println!("My Tuples is: {}",var_length); ------>>This will Print the length of The First Elemnt in the Tuple..
    






    //_________________________________________NESTED TUPLES_____________________________________________

    // let nest_tp =(5,4.01,(1,"Khan"),31);
    // println!("The Nested Tuples is :{}  ", nest_tp.2.1);

    // // EMPTY TUPLES
    // let empty_tp =();




    //_________________________________________ARRAY______________________________________________________
    /*
       -->> In array we have the similar types of elements.
       -->> in most IDE we have automatically the size and type option is present
       Syntax for array is:
        let mut array name :type + size = [elements];
    */
    // let mut new_array_size = [4,2,4,7,6];
    // new_array_size[2] as i32;
    // new_array_size[3] =10;
    // let lg =new_array_size.len();

    //  println!("The Array  is : {:?}",new_array_size);  // Here The {:?} Shows that the Complex data.../or Not simple scaler from the Array

    // let arr:[i32;0] =todo!();
    // println!("The Array with same Elemnts is : {:?}",arr.len());

    // let practice =[1,4,5,7,8,4,1,10];
    // println!("The Slicing is: {:?}",std::mem::size_of_val(&practice)); // slicing of an Array............>>No Bit return__32
    // let chdex =practice.get(4);
    // println!("The {:?}",chdex.unwrap());

    //___________________________________________Note______________________________________________________
    /*
       with the .get method we get the value at that index if exist..if not then it will return the None..
    */

    //___________________________________________Vector____________________________________________________
    /*
       vector is same like Array but the key point is " it is resizebale / means not fixed in size like array"
       ..and the rust developer using vector very widely
       -->> In vector we have same types of elements like array.
       syntax:
       let mut vector_name = vec![1,2,4,54,75,2];





       --------------------------------------------------------------------------------------------------
       |   if Array out of index it will compile and return with "None".                                 |
       |   But if Vector out of index ...it will compile but with No value except with the use of .get() |
       __________________________________________________________________________________________________|





    */
    // let  monday:Vec<i32> = vec![1,4,5,6,4];
    // println!("Our Vector is: {:?}",monday.get(5));

    // let mut stringe_vector:Vec<&str> =vec!["Apple","mango","Apricot"];
    // stringe_vector[1]="banana";
    // println!("{:#?}",stringe_vector[3]);

    // let ano_ve:Vec<&str> =vec!["apricot";5];
    // println!("{:#?}",ano_ve);

    //________________________________FUNCTIONS__________________________________
    /*
       syntax: without_output
       fn function_name(){
           //code here...
       }
       with_outputs:
       fn function_name()->(return type i.e i32,f64 etc)
       {
           // code here...
       }
    */

    // _____________CALLING FUNCTION from main_______________________________

    // basics_fn();
    // addition_wala();
    // let ans=new_try(4, 4);
    // println!("The Answer is :{}",ans);



    //______________________First_Method________________________________
    // let(mul ,add,sub,st)= with_multiple_outputs(4, 2,"Khan");
    // println!("mul = {} Add = {} sub = {} st = {}",mul,add,sub,st);

    /*
    ______________________2ND_Method_____________________________________
        let result= with_multiple_outputs(4, 2,"Khan");
        println!("mul = {} Add = {} sub = {} st = {}",result.0 ,result.1,result.2,result.3);
    */
   //___________________________End of Fuction calling___________________________




   //_______________________________Taking input From User_________________________________
    //    use std::io::stdin()
    // print!("");

   
    // let mut guess = String::new();
    // println!("Enter a name:");
    
    //        std::io::stdin()                                                             //--->Need this library and the output of .read_line is Result Enum.
    //        .read_line(&mut guess)
    //        .expect("Failed to readline");                                               // .trim() Remove the whitespaces from the user inputs i.e(" Khan   ").>>("Khan");
    //         let guess:f64 =guess.trim().parse().expect("Invalid shy dy Entre Ko!");     // .parse() for converting to int from string.
    //        println!("You Entered:  {}", guess);








    //________________________________________RUST_OWNERSHIP_RULES____________________________________________
       /*
        . Each Value in rust has a variable called OWNER.
        . Only one Owner at a Time,means we cant Assign one value to multiple Variable or Owner.
        . When the Owner become out of Scope the Vlue become also Drop/loss.
       */

      /*
       Some more Important Points:
        . Premative
            . cant be Empty and have Fixed size
            . Easy To handle
            . Stack Allocated
            . Premative we have i.e Integers,bool,flaot, char, arrays
            . in Premative we have the Copy operation when the compiler see the assignment Operator.
        . Non Premative
            . Can be Empty and in Grow in size
            . difficult to handle
            . Heap Allocated
            . Non Premative we have i.e String,Vectors Etc.
            . In Non Premative we have the Move operation. 
       */


        // Ownership Rules_01_Example:
        // let a= 32;                          //No erroe Bcoz of premative and the copy operation take place.
        // let b = a;
        // println!("The value is : {} {}",a,b);




        // let a =String::from("abc");
        // let b =a;                                 // Because the Move Operation take place.(Non Premative) so just put this line let b= &a;
        // println!("The a {} and b {}", a,b);



        // let a:Vec<i32>=vec![1,2,4,3];
        // let b =&a;
        // println!("{:?}  {:?}",a,b);



        // {
        //     let my_name ="Khan";             // Example of Rule_03.
        //out_of scope th my_name variable;
        // }
        // println!("my_name is {}",my_name );








    //______________________________Checking_the _state of_OF heap and_stack_______________________________

    // let new_var =32;
    // Stack_number(new_var);
    // println!("The value inside of new_var in main: {}",new_var);




    //________________________________________________________________________________________________________

    /*References Rules  
             - One mutable reference in a scope
             - Many immutable references
             - Mutable and immutable can not coexist
             - Scope of a reference 
             - Data should not change when immutable references are in scope 

    */

    //___________________________________Mutable and immutable References___________________________________


    // Rules 01 Example :
                // .  No more then one Mutable refrence is Allowed.
    // let mut var =vec![1,2,4,5];
    // let mut var1 =&mut var;
    // let mut var2 =&mut var;
    // println!("The value is var1 {:?} and var2 {:?}",var1,var2);


    // Rules 02 Example :
                // .   more then one or many  IMMutable refrence is Allowed
    // let mut var =vec![1,2,4,5];
    // let  var1 =&var;
    // let var2 =&var;
    // println!("The value is var1 {:?} and var2 {:?}",var1,var2);



    //Example 03 a
    // let mut s1 = String::from("abc");
    // let s2 = &s1;
    // println!("The value s1 ={} and s2 ={}",s1, s2);
    // let s3 =&mut s1; ------------------->>>> This Line Cause Error Because the  &mut.
    // println!("The s3 {} {}",s2,s3);

    //Example 03 b
    // let mut  s1 =String::from("abc");
    // let s2 =&s1;
    // println!("s1: {} ,s2: {}",s1,s2);
    // let  s3 = &mut s1;
    // println!("s1: {} ,s2: {}, s3: {}",s1,s2,s3);
    // println!("s: {} ",s3);




    //____________________________________IF else and if let syntax and Example_____________________________________________

    // println!("Enter Your Number: ");
    // let mut num=String::new();

    // std::io::stdin()
    // .read_line(&mut num)
    // .expect("Enter Valid Number Here");
    // let num:i32 =num.trim().parse()
    // .expect("Is it the Right Number ?");

    // let greater = if num > 10{
    //     println!("Num is Greater")
    // }
    // else if num < 10{
    //     println!("Number is Less then 10")
    // }
    // else
    // {
    //     println!(" Number is Equal")
    // };
    // println!("The Number is :{:?}",greater);



    //__________________________________________IF Let___________________________________________________


    // let n = -5;

    //     if n < 0 {
    //         print!("{} is negative", n);
    //     } else if n > 0 {
    //         print!("{} is positive", n);
    //     } else {
    //         print!("{} is zero", n);
    //     }

    //     let big_n =
    //         if n < 10 && n > -10 {
    //             println!(", and is a small number, increase ten-fold");

    //             // This expression returns an `i32`.
    //             10 * n
    //         } else {
    //             println!(", and is a big number, halve the number");

    //             // This expression must return an `i32` as well.
    //             n / 2
    //             // TODO ^ Try suppressing this expression with a semicolon.
    //         };
    //     //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    //     println!("{} -> {}", n, big_n);

    //______________________________________________IMPLEMENTATION OF MATCH________________________________

    // let mut var1 =String::new();
    // std::io::stdin()
    // .read_line(&mut var1)
    // .expect("Enter Valid Argument");
    // let  var1 :i32 =var1.trim().parse().expect("Argument is Not Valid");

    // match var1 {
    //     1..=10=> println!("var1 is B/W 1 and 10"),
    //     11..=20=> println!("var1 is B/W 11 nad 20"),
    //     _=> println!("var1 is Not in given Range"),
        
    // }


    // let x = Some(1);
    // let y = 10;

    // match x {
    //     Some(5) => println!("Got 50"),
    //     Some(y) => println!("Matched, y = {y}"),
    //     _ => println!("Default case, x = {:?}", x),
    // }

    // println!("at the end: x = {}, y = {}", x.unwrap(),y);





    //___________________________________L O O P IN  R U S T !!___________________________________________


    // Loop keyword
    // let mut count =0;
    // loop{
    //     count = count +1;
    //     if count == 5{
    //         continue;
    //     }
    //     print!("{} ", count);
    //     if count ==10 {
    //         break;
    //     }
    // }


    // for loop in Rust
    // for i in 1..6 {
    //     println!("{}", i);
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // assert_eq!(result, 20);



    //______________________________________________________Varients of the Person struct from the main________________________________
    // let mut p1 = Person{             // if instance(p1) is mutable then we can assign  another value to it.
    //     name:String::from("Fazal Sultan"),
    //     citizenship:String::from("pakistan"),   
    //      salary:400,
    //      age:23,
    //      gender:'M'
        
    //     };
    //     p1.citizenship =String::from("Canada");  // This Is the Updated value!! Now This Will be printing...
    //     println!("The Name is: {}",p1.name);
    //     println!("The Country is: {}",p1.citizenship);
    //     println!("The Gender is: {}",p1.gender);
    //     println!("The Salary  is: {}",p1.salary);
    //     println!("The Age is: {}",p1.age);


    //     println!("The Function return Value is : {}", p1.compute_tax());
    //     // 2nd Method is :
    //     println!("The Function with the second method is : {}",Person::compute_tax(&p1));
    //     let p2 =Person{
    //         citizenship:String::from("China"),
    //         age:21,
    //         salary:451,
    //         name:String::from("khan"),
    //         gender:'F'
    //     };
    //     println!("The p2 is: {}",p2.citizenship);



    //________________________________The trait Structure___________________>>><<<Intailzation from here________<<>>
    // let t1=Teacher{
    //     subject:String::from("Advance programming"),
    //     enroll_student:30,
    //     salary:4500,
    //     office_no: 10,
    //     name:String::from("Sultan")

    // };

    // println!("These are the values : {:?}",Teacher::teacher_info(&t1));
    // println!("These are the values : {:?}",Teacher::student_info_1(&t1));


    //_____________________________Enums____________________related_Things__________________________Here:


    // let represnt_colour = (colour_names::black,colour_names::blue,colour_names::green,colour_names::white,colour_names::red);
    // let rep_colour =colour_names::Black;
    // println!("The represnted_colour is : {:?}",rep_colour as i32);


    //________________________________________________Generics calls From Here____________________:
    // println!("The Try Fun will return {}",Try(5));
    // println!("The Try Fun will return {}",Try(5.5));
    // let s1 =Sultan{
        
    //     x:5,
    //     y:3,
    //     r :2.0
    // };
    // println!("the values are from s1 : {} {} {}",s1.x, s1.y ,s1.r);



    // let s2 =Sultan{
        
    //     x:5.01,
    //     y:3
    // };
    // println!("the values are from s2 : {} {}",s2.x, s2.y);

    // let s3 =Sultan{
        
    //     x:5,
    //     y:3.02
    // };
    // println!("the values are from s3 : {} {}",s3.x, s3.y);

    // let s4 =Sultan{
        
    //     x:5.5,
    //     y:3.23
    // };
    // println!("the values are from s4 : {} {}",s4.x, s4.y);








    //_________________________________________________Life Time!!___________________________________________________
    //Life time Def: -->>" the life time Explain the Scope for which the reference is  valid...."
    // "borrow Checker" -->> Which Check The Life Time related Issues.-->>> At Compile Time.
    // "The Generic Life time perameter overcome the Two Issue (i) Diaglling Pointers (ii) undetermine Specifiers".
    // The Generic Life time Syntax is :
    // 'a and 'b further explain in leter example
    // let a= 4;
    // let b = 5; 

    // let con = greater_and_smaller(&a ,&b);
    // println!("The Value is : {:?}", con);




    //____________________________________________________Closure____________________________________________________

    /*
        Closure Def : Closure are just Like Function but Without no Names.
        Closure Syntax:
        |(Input goes Here)| { output will be Here}
        Important Note :OwnerShip Rules are also same like the Function.
    */

    // Simple Closure -->> Without Input and output....
    // let fyp =|| println!("We need Justice");
    // fyp();
    // let x =15;
    // let elction = | x |{println!("True")

    // };
    // elction(x);


    // let mut check =vec![0,1,2,4,5,7,8,1];
    // // println!("The Values are : {:?}",check.iter());
    // // println!("The Values are : {:?}",check.next().unrap());
    // let ok =check.iter().position(|&x| x >=1);
    // let ok =check.iter().rev();  
    // println!("The Probkem is Here :{}",check.next());                                //->>it will check all the elements and return True or false if all value statisfies the condition..all()
    // print!("The Output is : {:?}", ok);                            //--->> it will check all the Element but if any elements is true it will return True..any()
    //                                                                         //it will check  the elemnts from left to right if the condition TRUE it will returnm SOME VALUE.


    // let Ik  =check.iter().find(|&&x| x > 5);
    // println!("Th Find Will return : {:?}",Ik.unwrap());

    // let a = [1, 2, 3];
    // let mut iter = a.iter();
    // println!("the {:?}",iter);
    // let ck= iter.next();
    // println!("The Next value is  {:?}",iter.next().unwrap());










        //______________________________________Ratta waly Questiond_________________________
        // rust --> pkg--> crates contain Modules --> i) binary crates ii) library crates:
        // binary Crates -->> Is a Code which can Execute...
        // Library Crates -->> which can be use  by the Other Programs...





        //__________________________________Smart_Pointers___________________________________
        // they are NOT just like the Normal Pointers, but they may also contain some meta data and some capabalities.
        // in Rust WE have evry thing on The Stack Memory But the Smart Pointers to reference data store in The Heap Memory.




        //________________________________________Basic macros call From Here_____________________________________
        // try_macros!();


}
//_______________________________________________Structure in Rust_______________________________________________
//_______________________________________________Life Time Related Functions_____________________________________


// fn greater_and_smaller<'a ,'b>(a: &'a i32  ,b: &'b i32 )->(&'a i32, &'b i32){
//     if a > b{
//         a
//     }else {
//         b
//     }
// }








/*  
                                          Struct Basics
                                          -   We can store different Data types in struct.
 */

// struct Person {
//     name :String,
//     citizenship: String,
//     salary :i32,
//     age :i32,
//     gender :char

// }
// impl Person {                                             // Here the Impl keyword followed by the struct name means that we are implementing this function on the above struct.
//     fn compute_tax(&self)->i32{
//         self.salary/ 10
//     } 
// }




/*
                                       Trait in rust: ------------->>A trait in Rust is a group of methods that are defined for a particular type.
                                       Trait are just like the Abstract class in C++ or interfaces in Java..
 */

// struct Teacher{
//     subject:String,
//     name:String,
//     enroll_student :i32,
//     office_no:i32,
//     salary: i32
// }



// struct  Student{
//     name:String,
//     feild_of_study: String,
//     eraned_credit:i32,
//     Cgpa:f64,
//     semester:i32
// }



// trait CombineInformation {
//     fn teacher_info(&self)->(String,String,i32,i32,i32);
//     fn student_info_1(&self){
//         println!("Ok I am here Ahhhh");
//     }
// }





// impl CombineInformation for Teacher {
//     fn teacher_info(&self)->(String,String,i32,i32,i32) {
//         (self.subject.clone(),self.name.clone(),self.enroll_student,self.office_no,self.salary)
//     }
    // fn student_info_1(&self)->(String,String) {
    //     (self.subject.clone(),self.name.clone())
    // }
//}





// impl CombineInformation for Student {
//     fn teacher_info(&self)->(String,String,i32,f64,i32) {
//         (self.name.clone(),self.feild_of_study.clone(),self.eraned_credit,self.Cgpa,self.semester)
//     }
//     fn student_info_1(&self)->(String,String) {
//         (self.name.clone(),self.feild_of_study.clone())
//     }
// }




//_____________________________Enum in Rust______________________________programming Langauge___:
//An enum in Rust is a type that represents data that is one of several possible variants. Each variant in the enum can optionally have data associated with it:


//Enum is a custom data type that is composed of variants.
// Variants are values which are definite..
// enum colour_names {
//     Blue,
//     Green,
//     White,
//     Black,
//     Red
// }




// fn cal_square<T:std::ops::Mul<Output =T>+Copy >(x:T)->T{
//     x*x
// }







//_____________________________Generics in rust(Function + Struture)
// fn Try <T>(x: T)->T 
// where T: std::ops::Mul<Output = T>+ Copy + std::ops::Add<Output = T>{
//     x+x;
//     x*x
    
// }









//_______________________Generics Structure ________________________
// struct Sultan<T ,U ,Z>{
//     x: T,
//     y :U,
//     r : Z
// }
// ______________________________FUNCTIONS/METHODS_____________________________________

// fn basics_fn()              // Without_outputs & inputs/Arguments...
// {
//     println!("Hello this is my first Function");
// }

// fn addition_wala()
// {
//     let var1=16;
//     let var2=16;
//     let var3= var1 + var2;
//     println!("The result is :{}",var3);
// }

// fn new_try(var1:i32 , var2:i32)->i32
// {
//     var1*var2              // no need of semi colon here because it just return the value..if u wana put the compiler will complain..
// }

// the following function(with_multiple_outputs) return mutiple values with different types in the forms of Tuples..

// fn with_multiple_outputs(var1:i32,var2:i32,var3:&str)->(i32,i32,i32,&str)
// {
//     (var1*var2,var1+var2,var1-var2,var3)
// }









//______________________________Checking_the _state of_OF heap and_stack_from Here_______________________________
// &mut means to change the Value but not the Owner...

// fn Stack_number(mut new_var)
// {
//     new_var=16;

//     println!("The Value inside Function of New_Var is: {}",new_var);

// }



/*
    _____________________________________________Reference Rules_____________________________________________________
    
 */



/*
    Basics Micros:
    The macros provide a way of code thats write other codes! --->>>aldo called Meta Programming.
    Macros have 2 types :
    1. Macro with a name and a body.
    2. Macro with a name and a list of parameters.

    i) :Declerative Macros
    ii) :Procedural Macros
         -->>Costome Derived,attribute derived, and Function
    i) : Declerative Macros:
           . macros by example, or maros rules, or plane macros
    syntax :
    macro_rules! macro_name{
        (...)=> {...};
        (...)=> {...};
        (...)=> {...};
        (...)=> {...};

    }


    captured can be :
    a type, a expressiion and a indentifiers.


 */
// #[macro_use(try_macros())]
// macro_rules! try_macros {
//     () => {
//         1+1;
//     };
// }