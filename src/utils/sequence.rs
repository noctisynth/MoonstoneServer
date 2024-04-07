use anyhow::{Ok, Result};
use moonstone_db::operations::account;
use rand::{rngs::OsRng, Rng};

/// 生成随机序列
///
/// `generate_sequence`要求生成一个随机的且不存在相同值的序列。
/// 此函数遵循旧日协定，它的产出值与旧日序列的等效。
pub(crate) async fn generate_sequence() -> Result<String> {
    loop {
        let sequence = OsRng.gen_range(1000..9999).to_string();
        let account_find = account::get_by_sequence(&sequence).await?;
        if account_find.is_none() {
            return Ok(sequence);
        }
    }
}
