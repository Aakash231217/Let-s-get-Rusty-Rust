fn main() {
    println!("Hello, code eaters!");
    //u8 unsigned integer
    let num:u16=256;
    println!("this is stored in num {}",num);

    // now &str(fixed length)  (mut) if you want mutability
    let string_literal= "Hi code eaters";
    println!("This is string literal{}",string_literal);
    //string - Dynamic strings - Heap Allocated
    let mut str_lit = String::from("Hi coders");
    str_lit.push_str("What's up?");
    println!("This is string {}",str_lit);


    //tupple
    let emp_info:(&str,u8) = ("Aakash",50);
    let emp_name = emp_info.0;
    let emp_age= emp_info.1;


    //destructuring
    let (employee_name,employee_age)=emp_info;
    println!("Employee name is{}{}",emp_name,emp_age);
    println!("Employee details{}{}",employee_name, employee_age);
    print_value(5);
    let num1:u8=10;
    let num2:u8=20;
    let result:u8 = add(num1 ,num2);
    println!("The sum of num1 and num2 is{}",result);
}

fn print_value(item:u8){
    println!("{}",item);
}

fn add(item1:u8, item2:u8)->u8{
    return item1+item2;
}
