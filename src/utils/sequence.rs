use entity::account::{Column as AccountColumn, Entity as Account};
use rand::{rngs::OsRng, Rng};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// 生成随机序列
///
/// `generate_sequence`要求生成一个随机的且不存在相同值的序列。
/// 此函数遵循旧日协定，它的产出值与旧日序列的等效。
pub(crate) async fn generate_sequence(db: &DatabaseConnection) -> i32 {
    let mut rng = OsRng;

    loop {
        let random_number: i32 = rng.gen_range(1000..9999);
        let account_find = Account::find()
            .filter(AccountColumn::Sequence.eq(random_number))
            .one(db)
            .await
            .unwrap();
        if account_find.is_none() {
            return random_number;
        }
    }
}
