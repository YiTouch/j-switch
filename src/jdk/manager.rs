use crate::config::{Config, JdkInfo}; // [注释] 引入Config配置类和JdkInfo信息结构体
use crate::error::{JdkError, Result}; // [注释] 引入自定义错误类型和Result别名
use crate::jdk::detector::JdkDetector; // [注释] 引入JdkDetector类，用于检测和扫描系统JDK

pub struct JdkManager { // [注释] 定义公共结构体JdkManager，JDK管理器，封装JDK的核心操作
    config: Config, // [注释] 私有字段config，存储JDK配置信息
}

impl JdkManager { // [注释] 为JdkManager结构体实现方法
    pub fn new() -> Result<Self> { // [注释] 公共关联函数，创建新的JdkManager实例，加载现有配置
        let config = Config::load()?; // [注释] 从磁盘加载配置文件，如果失败则传播错误
        Ok(Self { config }) // [注释] 构造JdkManager实例并返回，使用字段初始化简写
    }

    pub fn config(&self) -> &Config { // [注释] 公共方法，返回配置对象的不可变引用
        &self.config // [注释] 返回config字段的引用，不转移所有权
    }

    pub fn config_mut(&mut self) -> &mut Config { // [注释] 公共方法，返回配置对象的可变引用，允许修改
        &mut self.config // [注释] 返回config字段的可变引用
    }

    /// Scan and update JDK registry
    pub fn scan_jdks(&mut self) -> Result<Vec<JdkInfo>> { // [注释] 公共方法，扫描系统中的所有JDK并更新注册表
        let detected = JdkDetector::detect_all()?; // [注释] 调用JdkDetector扫描系统中所有JDK，返回Vec<JdkInfo>
        
        // Update config with newly found JDKs
        for jdk in &detected { // [注释] 遍历检测到的每个JDK信息
            let key = jdk.version.clone(); // [注释] 克隆版本号作为HashMap的键
            if !self.config.jdks.contains_key(&key) { // [注释] 检查该版本是否已经在配置中存在
                self.config.add_jdk(key, jdk.clone()); // [注释] 如果是新发现的JDK，添加到配置中
            }
        }
        
        self.config.save()?; // [注释] 将更新后的配置保存到磁盘
        Ok(detected) // [注释] 返回检测到的JDK列表
    }

    /// Get all registered JDKs
    pub fn list_jdks(&self) -> Vec<(&String, &JdkInfo)> { // [注释] 公共方法，获取所有已注册JDK的排序列表
        let mut jdks: Vec<_> = self.config.jdks.iter().collect(); // [注释] 将HashMap的迭代器收集为Vec，包含(版本号, JdkInfo)元组的引用
        jdks.sort_by(|a, b| { // [注释] 使用自定义比较函数对JDK列表排序
            // Try to parse as numbers for proper sorting
            let a_num: std::result::Result<u32, _> = a.0.parse(); // [注释] 尝试将第一个版本号解析为无符号整数
            let b_num: std::result::Result<u32, _> = b.0.parse(); // [注释] 尝试将第二个版本号解析为无符号整数
            
            match (a_num, b_num) { // [注释] 匹配解析结果的组合
                (Ok(a), Ok(b)) => a.cmp(&b), // [注释] 如果两个都解析成功，按数值比较（如8 < 11 < 17）
                _ => a.0.cmp(b.0), // [注释] 否则按字符串字典序比较
            }
        });
        jdks // [注释] 返回排序后的JDK列表
    }

    /// Get current active JDK
    pub fn get_current(&self) -> Option<&JdkInfo> { // [注释] 公共方法，获取当前激活的JDK信息
        self.config.get_current() // [注释] 委托给Config的get_current方法，返回Option<&JdkInfo>
    }

    /// Get current JDK version key
    pub fn get_current_version(&self) -> Option<&String> { // [注释] 公共方法，获取当前JDK的版本号字符串
        self.config.current_jdk.as_ref() // [注释] 将Option<String>转换为Option<&String>，返回版本号引用
    }

    /// Switch to a specific JDK version
    pub fn switch_jdk(&mut self, version: &str) -> Result<&JdkInfo> { // [注释] 公共方法，切换到指定版本的JDK
        let jdk = self.config.get_jdk(version) // [注释] 从配置中获取指定版本的JDK信息
            .ok_or_else(|| JdkError::JdkNotFound(version.to_string()))?; // [注释] 如果找不到，返回JdkNotFound错误
        
        // Verify JDK still exists
        if !JdkDetector::is_valid_jdk(&jdk.path) { // [注释] 验证JDK路径是否仍然有效（文件可能已被删除）
            return Err(JdkError::InvalidPath(format!( // [注释] 如果路径无效，返回InvalidPath错误
                "JDK path no longer valid: {}",
                jdk.path.display() // [注释] 显示无效的JDK路径
            )));
        }
        
        self.config.set_current(version.to_string()); // [注释] 在配置中设置当前激活的JDK版本
        self.config.save()?; // [注释] 保存更新后的配置到磁盘
        
        Ok(self.config.get_jdk(version).unwrap()) // [泣释] 返回切换后JDK的信息引用，unwrap安全因为前面已验证存在
    }

    /// Save configuration
    pub fn save(&self) -> Result<()> { // [注释] 公共方法，保存配置到磁盘
        self.config.save() // [注释] 委托给Config的save方法
    }
}
