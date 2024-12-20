use color_eyre::eyre;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use entity::{registration, registration::Entity as Registration};

use super::util::run_async_block;
use crate::cli::args::Reg;
use crate::seaorm_setup::create_db_connection;

async fn list_regs() -> eyre::Result<()> {
    let db = create_db_connection().await?;
    // TODO make connection singleton
    let regs = Registration::find()
        .order_by_asc(registration::Column::Id)
        .all(&db)
        .await?;
    for reg in regs {
        println!("{}", serde_json::to_string(&reg)?);
    }
    Ok(())
}

async fn delete_reg(uuid: &str) -> eyre::Result<()> {
    let db = create_db_connection().await?;
    let res = Registration::delete_many()
        .filter(registration::Column::Uuid.eq(uuid))
        .exec(&db)
        .await?;
    match res.rows_affected {
        1 => println!("Deleted registration {}", uuid),
        0 => println!("Registration not found"),
        _ => unreachable!(),
    }
    Ok(())
}

pub fn main(regs: Reg) -> eyre::Result<()> {
    use Reg::*;
    match regs {
        List => {
            run_async_block(list_regs())?;
            // let regs = crud::get_all_registrations(&mut conn)?;
            // for reg in regs {
            //     println!("{}", serde_json::to_string(&reg)?);
            // }
        }
        Delete(reg_delete) => {
            run_async_block(delete_reg(&reg_delete.uuid))?;
        }
    }
    Ok(())
}
