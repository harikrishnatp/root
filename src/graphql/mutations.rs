use async_graphql::{Context, Object};
use chrono::{NaiveDate, NaiveTime};
use sqlx::PgPool;
use  sqlx::types::chrono;
use std::sync::Arc;

use crate::db::{member::Member, attendance::Attendance};

pub struct MutationRoot;

#[Object]
impl MutationRoot {

    //Mutation for adding members to the Member table
    async fn add_member(
        &self, 
        ctx: &Context<'_>, 
        rollno: String, 
        name: String, 
        hostel: String, 
        email: String, 
        sex: String, 
        year: i32,
        macaddress: String,
    ) -> Result<Member, sqlx::Error> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool not found in context");

        let member = sqlx::query_as::<_, Member>(
            "INSERT INTO Member (rollno, name, hostel, email, sex, year, macaddress) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
        .bind(rollno)
        .bind(name)
        .bind(hostel)
        .bind(email)
        .bind(sex)
        .bind(year)
        .bind(macaddress)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(member)
    }

    
    //Mutation for adding attendance to the Attendance table
    async fn add_attendance(
        &self,
        ctx: &Context<'_>,
        id: i32,
        date: NaiveDate,
        timein: NaiveTime,
        timeout: NaiveTime,
        is_present: bool,
    ) -> Result<Attendance, sqlx::Error> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool not found in context");

        let attendance = sqlx::query_as::<_, Attendance>(
            "INSERT INTO Attendance (id, date, timein, timeout, is_present) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(id)
        .bind(date)
        .bind(timein)
        .bind(timeout)
        .bind(is_present)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(attendance)
    }
    
    async fn mark_attendance(
        &self,
        ctx: &Context<'_>,
        id: i32,
        date: NaiveDate,
        is_present: bool,
    ) -> Result<Attendance,sqlx::Error> {
        let pool = ctx.data::<Arc<PgPool>>().expect("Pool not found in context");

        let attendance = sqlx::query_as::<_, Attendance>(
            "UPDATE Attendance SET is_present = $1 WHERE id = $2 AND date = $3 RETURNING *"
        )
        .bind(is_present)
        .bind(id)
        .bind(date)                                                                                                                                                                         
        .fetch_one(pool.as_ref())
        .await?;                            

        Ok(attendance)
    }
}
