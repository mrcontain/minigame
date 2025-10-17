/*
 * @Author: error: error: git config user.name & please set dead value or install git && error: git config user.email & please set dead value or install git & please set dead value or install git
 * @Date: 2025-04-16 14:47:08
 * @LastEditors: error: error: git config user.name & please set dead value or install git && error: git config user.email & please set dead value or install git & please set dead value or install git
 * @LastEditTime: 2025-04-28 17:41:30
 * @FilePath: /oxide/src/config.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE_l
 */
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// 绑定配置
#[derive(Debug, Deserialize, Serialize)]
pub struct BindConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyConfig {
    pub private_key: String,
    pub public_key: String,
}

// 主配置结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub bind: BindConfig,
}

impl Config {
    /// 加载yaml配置文件
    pub fn try_load() -> Result<Self> {
        Self::load_from_file("config.yaml")
    }

    /// 从指定路径加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(&path)
            .with_context(|| format!("无法打开配置文件: {}", path.as_ref().display()))?;
        let reader = BufReader::new(file);

        let config = serde_yaml::from_reader(reader).with_context(|| "解析YAML配置文件失败")?;

        Ok(config)
    }

    /// 获取绑定地址
    pub fn get_bind_address(&self) -> &str {
        &self.bind.ip
    }

    /// 获取绑定端口
    pub fn get_bind_port(&self) -> u16 {
        self.bind.port
    }

    /// 获取完整的绑定地址（包含IP和端口）
    pub fn get_bind_socket(&self) -> String {
        format!("{}:{}", self.get_bind_address(), self.get_bind_port())
    }

    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        serde_yaml::to_writer(file, self)?;
        Ok(())
    }
}
