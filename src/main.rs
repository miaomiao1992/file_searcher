use anyhow::{Result, Context};
use anyhow::anyhow;

use std::any::Any;
use std::io::Read;
use std::fs;

/**递归获取某路径下所有子文件（含文件夹）
 *@created 2022-01-15
 *@update 2022-01-15 添加regex依赖
 *@update 2022-01-16 完善，通过添加anyhow依赖增加显示栈调用信息
 *@update 2022-01-17 完善，添加新功能，开始写正则表达式
 *@update 2022-01-23 继续玩阿寒啊
*/

fn all_path(path: &String) -> anyhow::Result<Vec<String>>
{
    let mut path_list: Vec<String> = Vec::new();
    if fs::metadata(&path)?.is_file()
    {
        path_list.push(path.clone());
    }
    if fs::metadata(&path)?.is_dir() {
        for child_dir in fs::read_dir(&path)? {
            let mut child_path_list =
                all_path(&String::from(
                    child_dir?.path().
                        as_os_str().to_str()
                        .ok_or(anyhow!( "获取某路径下文件名列表失败"))?
                ))?;
            path_list.append(&mut child_path_list);
        }
    }
    return Ok(path_list);
}

//作废
/*
fn all_path2(root_path: &String) -> Result<Vec<String>, Box<anyhow:: Error>> {
    let mut path_list = vec![String::from(root_path)];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        for index in start_index..path_list.len() {
            let path = &path_list[index];
            if fs::metadata(path)?.is_dir() { for child_dir in fs::read_dir(&path)? { path_list.push(String::from(child_dir?.path().as_os_str().to_str().expect(""))); } }
        }
        if list_len == start_index { break; }
        start_index = list_len;
    }
    return Ok(path_list);
}
*/
fn main() {
    println!("欢迎使用文件名搜索小助手!");

    loop {
        run().unwrap_or_else(|err| {
            println!("异常信息：{:?}", err);
        });
    }
    std::process::Command::new("cmd.exe").arg("/c").arg("pause").status();
}

use regex::{Regex, Captures};

/**
 *验证一些文件名是否满足某个正则表达式
 *
 */
fn validate_fileName(reg: &String, file_list: Vec<String>) -> anyhow::Result<Vec<String>>
{
    let mut file_list_ok: Vec<String> = Vec::new();
    let regex = Regex::new(reg)?;
    let mut flag=true;
    for x in 0..file_list.len()
    {
        if let Some(file) = file_list.get(x) {
            if regex.is_match(file) {
                file_list_ok.push(file.clone());
                flag=false;
                println!("文件已找到！文件名及路径（./表示本程序所在目录）为：{}", file);
            }
        }
    }
    if flag
    {
        println!("很可惜，文件名及路径未找到！");
    }

    return Ok(file_list_ok);
}

fn run() -> Result<(), anyhow::Error>
{
    println!("请输入需要遍历搜索的文件名[和文件夹路径]：");
    let mut path = String::new();
    std::io::stdin().read_line(&mut path)?;
    path = path.replace("\r", "").replace("\n", "");

    let separator = Regex::new(r"([ ]+)")?;
    let str_list: Vec<&str> = separator.split(path.as_str()).collect();
    let reg = str_list.get(0).ok_or(anyhow!( "获取文件名形式错误"))?.to_string();
    path = str_list.get(1).unwrap_or(&&String::from("./").as_str()).to_string();
    let path_list = all_path(&mut path).or_else(|err| {
        return Err(err);
    })?;

    validate_fileName(&reg, path_list)?;
    return Ok(());
}