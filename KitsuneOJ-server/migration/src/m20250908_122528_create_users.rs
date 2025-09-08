use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid() // UUID
                            .not_null()
                            .primary_key() // PK 지정
                            .default(Expr::cust("gen_random_uuid()")), // 자동 생성
                    )
                    .col(ColumnDef::new(Users::Handle).text().not_null().unique_key()) // 핸들, UNIQUE
                    .col(ColumnDef::new(Users::DisplayName).text().not_null()) // 이름
                    .col(ColumnDef::new(Users::Bio).text().null())
                    .col(string_len(Users::Email, 254).not_null().unique_key()) // 이메일, UNIQUE, RFC 표준 - 254
                    .col(ColumnDef::new(Users::Password).text().null()) // 비밀번호 해시, OAUTH 지원 시 NULL 가능
                    .col(
                        ColumnDef::new(Users::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::IsBanned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Users::ProfileImage).text().null())
                    .col(ColumnDef::new(Users::BannerImage).text().null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await?;

        // handle 컬럼 인덱스 생성 (로그인/검색 성능 최적화)
        manager
            .create_index(
                Index::create()
                    .name("idx_users_handle")
                    .table(Users::Table)
                    .col(Users::Handle)
                    .to_owned(),
            )
            .await?;

        // email 컬럼 인덱스 생성 (로그인/검색 성능 최적화)
        manager
            .create_index(
                Index::create()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Handle,
    DisplayName,
    Bio,
    Email,
    Password,
    ProfileImage,
    BannerImage,
    IsVerified,
    IsBanned,
    CreatedAt,
}