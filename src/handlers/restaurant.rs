use sqlx::{self, Pool, Postgres, types::{Uuid, time::OffsetDateTime}, FromRow};

#[derive(Debug, FromRow)]
pub struct Restaurant {
  pub id: Uuid,
  pub name: String,
  pub address: String,
  pub email: String,
  pub phone: String,
  pub created_at: OffsetDateTime,
  pub updated_at: OffsetDateTime
}

pub struct NewRestaurant {
  pub name: String,
  pub address: String,
  pub email: String,
  pub phone: String,
}

type DBResult<T> = Result<T, Box<dyn std::error::Error>>;

impl Restaurant {
  pub async fn all(pool: &Pool<Postgres>) -> DBResult<Vec<Restaurant>> {
    let restaurants = sqlx::query_as!(
      Restaurant, 
      "select * from restaurants"
    ).fetch_all(pool).await?;
    Ok(restaurants)
  }

  pub async fn get(pool: &Pool<Postgres>, id: &str) -> DBResult<Restaurant> {
    let parsed_id = Uuid::parse_str(id)?;
    let restaurant = sqlx::query_as!(
      Restaurant,
      "select * from restaurants where id= $1",
      parsed_id
      ).fetch_one(pool).await?;
    Ok(restaurant)
  }

  pub async fn create(pool: &Pool<Postgres>,restaurant_details: NewRestaurant) -> DBResult<Restaurant> {
    let (
      name,
      address,
      email,
      phone
      ) = (
        restaurant_details.name, 
        restaurant_details.address,
        restaurant_details.email,
        restaurant_details.phone
      );
    
    let restaurant = sqlx::query_as!(
      Restaurant, 
      "insert into restaurants (name, address, email, phone) values ($1,$2,$3,$4) returning *",
      name,
      address,
      email,
      phone
    ).fetch_one(pool).await?;
    Ok(restaurant)
  }

  pub async fn destroy(
    pool: &Pool<Postgres>, 
    id: Uuid
  ) -> DBResult<()> {
    sqlx::query!(
      "delete from restaurants where id = $1",
      id
    ).execute(pool).await?;

    Ok(())
  }
}