use std::env;
mod database;
use database::Database;
#[warn(dead_code)]
fn main() {
    println!("Hello, let us go to finish a spectacular project!");
    let    ml_accpet:Vec<String> = env::args().collect(); //获取命令行参数
    //输出命令行接收器
       println!("output the args of command:{:?}",ml_accpet);
       //命令行接收器，第一个元素默认为程序的路径，以cargo run add xhl为例
       //cargo run 为程序命令。默认不计入向量中,add为第二个元素，xhl为第三个元素，以空格区分开
    if ml_accpet.len()<2{
        println!("Uaage: rodo [add|rm|ls] [ml_accpet]");
        return;         //当输入的指令格式是cargo run时,输出这里，提示指令的有效格式
    }
    let command = &ml_accpet[1];
    let mut db = Database::open("..accept");  
    match command.as_str() {
        "add"   =>{     //输入指令的长度大于3时，如cargo run add 时会提示该指令的正确信息
            if ml_accpet.len()<3{   //cargo run add
                println!("命令的标准格式为:rodo add [contents]");
                return;
            }
            let contents = &ml_accpet[2..].join(".");
          Database::add_record(&mut db,&contents);
           
        }
        "rm"   =>{
            db.show_database();
           let result =  db.remove();
           if let Err(err) = result {                     
            eprintln!("\x1b[31merror:\x1b[39m {}", err);
            std::process::exit(1);
        }
         
        }
        "ls"  =>{  
            db.show_database();}
        _ =>{
            println!("无效的指令，请检查后输入");
        }
    }
}
