use std::fs::{File,OpenOptions, self};
use std::io::{BufRead,BufReader,Write, Seek};
use std::io::{self};
pub struct Record{
   pub id:usize,
   pub event:String,
}
pub struct Database{
    pub file:File,
}
impl Database {
    pub fn open(filename:&str) -> Database {
        let file = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .read(true)
                                 .open(filename)
                                .unwrap();
                            Database{ file } 
        
    }
}
impl Database{  //增加一条记录
    pub fn add_record(db:&mut Database,shuru:&String){
             let    id = db.read_records().last().map(|r| r.id+1).unwrap_or(1);
             println!("id is {}",id);
                let record = Record { 
                        id, 
                        event: (shuru.clone()), };
            
            let   line = format!("{}.{}\n",record.id,record.event);
            writeln!(db.file,"{}",line).unwrap();
            println!("event added! :{}",record.event);
    }
}

// 解析记录行
impl From<&str> for Record {   //一个运用在Record上的特性
    fn from(line: &str) -> Self {
        let fields: Vec<&str> = line.split('.').collect();
        // 处理空行的情况
        if fields.len() == 1 {
            return Record {
                id: 0,
                event: "".to_string(),
            };
        }
        let event = fields[1..].join(".");
        Record {
            id: fields[0].parse::<usize>().unwrap(),
            event,
        }
    }
}
impl Database {   //读取记录
    pub fn read_records(&mut self) -> Vec<Record>{  //Record的一个向量
        let reader = BufReader::new(&self.file);
               reader
              .lines()
              .map_while(Result::ok)
              .filter(|line| !line.is_empty())
              .map(|line| Record::from(line.as_str()))
              .collect()
    }
}
impl Database {
    pub fn show_database(&mut self){
        let records = self.read_records();//Record的向量
        if records.is_empty(){
            println!("No record.You can add one with rodo add [content] ");
            std::process::exit(1);
        }
        for record in records{
            println!("* {}:{}",record.id,record.event);
        }

    }
    }
impl Database{
    pub fn remove(&mut self) -> Result<(), std::io::Error> {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        
            println!("请输入你所要删除的id");
            // 创建一个可读的缓冲区
            let mut input = String::new();
            // 从标准输入读取一行
            io::stdin().read_line(&mut input).expect("Failed to read line");
            // 移除末尾的换行符
            input.pop();                         
            // 将输入的字符串转换为整数
            let   id: usize = input.trim().parse().expect("Please type a number!");
            //找到要删除的line
        let line = lines.find(|(_, line)| {
            let record = Record::from(line.as_ref().unwrap().as_str());
            record.id == id
        });  
        match line {
            Some((i, _)) => {
                let contents = fs::read_to_string(".accept").unwrap();
                //生成一个不含待删除行的数据
                let new_contents = contents
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("\n"); 
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();
                Ok(())
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("No such record: {}", id),
            )),
            
        }
    }
}