use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CarLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CarLog::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CarLog::CarArrived)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CarLog::CarLeft)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(ColumnDef::new(CarLog::SpotId).not_null().small_integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CarLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CarLog {
    Table,
    Id,
    SpotId,
    CarArrived,
    CarLeft,
}
