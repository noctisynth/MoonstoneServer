use thiserror::Error;

#[derive(Error, Debug)]
pub enum Exception {
    #[error("未能在表[{table}]中查找到[{field}]为[{data}]的数据")]
    ColumnNotFound {
        table: String,
        field: String,
        data: String,
    },
    #[error("密码加密时出现异常: {error:?}")]
    PasswordHashFailed { error: argon2::password_hash::Error },
    #[error("包含[{feature}]特征的数据已经存在")]
    ColumnExists { feature: String },
    #[error("账户序列[{sequence}]使用密钥[{password}]身份验证失败")]
    AuthenticationFailed { sequence: String, password: String },
    #[error("权限标记未传入")]
    MissingPermissionField,
    #[error("时间戳错误")]
    TimestampError,
}
