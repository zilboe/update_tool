use std::fs;
use std::io;
use std::io::Read;
fn main() {
    println!("请选择你的选项:");
    println!("1.打开默认文件");
    println!("2.输入文件名打开文件");
    let user_default = "下个宇宙见.psd";

    let is_user_custom = get_user_choice();
    if is_user_custom {
        println!("用户选择自定义");
    } else {
        println!("用户选择默认");
        println!("默认文件为{}", user_default);
    }

    let mut s_read_buffer: Vec<u8> = Vec::new();

    let get_file_rong: bool = user_custom_handle(is_user_custom, &user_default, &mut s_read_buffer);
    if get_file_rong {
        let rong: u32 = s_read_buffer.iter().map(|&x| x as u32).sum();
        println!("文件冗余为:0x{:X?}字节", rong);
    }
    println!("请输入Enter关闭程序.......");
    io::stdin()
        .read_line(&mut String::new())
        .expect("读取命令行失败");
}

fn get_user_choice() -> bool {
    loop {
        let mut get_user_input = String::new();
        io::stdin()
            .read_line(&mut get_user_input)
            .expect("读取命令行失败");

        //清楚回车键
        match get_user_input.trim().parse() {
            Ok(choice) => match choice {
                1 => return false,
                2 => return true,
                _ => println!("请重新输入!"),
            },
            Err(_) => println!("解析失败,请重新输入!"),
        };
    }
}

fn user_custom_handle(user_select: bool, filename: &str, buffer: &mut Vec<u8>) -> bool {
    let file_name = if user_select {
        println!("请您输入打开的文件名:");
        let mut get_user_inputname = String::new();
        io::stdin()
            .read_line(&mut get_user_inputname)
            .expect("读取命令行失败");
        //解析输入
        String::from(get_user_inputname.trim())
    } else {
        String::from(filename)
    };
    let get_file_fs = fs::OpenOptions::new().read(true).open(&file_name);
    let mut get_file_fs = match get_file_fs {
        Ok(file) => file,
        Err(_) => {
            return false;
        }
    };

    let mut read_buffer: Vec<u8> = Vec::new();
    match get_file_fs.read_to_end(&mut read_buffer) {
        Ok(_) => {
            println!("读取文件成功.....");
            println!("文件大小为:{}字节", read_buffer.len());
            buffer.extend_from_slice(&read_buffer);
            read_buffer.len() > 0
        }
        Err(_) => false
    }
}
