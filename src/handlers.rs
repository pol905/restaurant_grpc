use sqlx::{Pool, Postgres, types::Uuid};
use tonic::{Request, Response, Status};
use crate::pb::{
  restaurant_service_server::RestaurantService,
  GetRestaurantRequest,
  GetRestaurantResponse,
  GetRestaurantsRequest,
  GetRestaurantsResponse, CreateRestaurantRequest, CreateRestaurantResponse, DeleteRestaurantRequest, DeleteRestaurantResponse
};

use self::restaurant::NewRestaurant;

mod restaurant;

#[derive(Debug)]
pub struct RestaurantHandler {
  pool: Pool<Postgres>
}

type ServiceResult<T> = Result<T, Status>;

impl RestaurantHandler {
  pub fn new(pool: Pool<Postgres>) -> Self {
    Self {
      pool
    }
  }
}


#[tonic::async_trait]
impl RestaurantService for RestaurantHandler {
    async fn get_restaurants(
      &self,
      _: Request<GetRestaurantsRequest>,
    ) -> ServiceResult<Response<GetRestaurantsResponse>> {
      let restaurants = restaurant::Restaurant::all(&self.pool).await;

      if restaurants.is_err() {
        return Err(Status::new(tonic::Code::Unavailable, "could not find restaurants"));
      }

      let restaurants = restaurants.unwrap();
      let restaurants = restaurants.into_iter().map(|restaurant| {
        GetRestaurantResponse {
          id: restaurant.id.to_string(),
          name: restaurant.name,
          address: restaurant.address,
          email: restaurant.email,
          phone: restaurant.phone,
          created_at: restaurant.created_at.to_string(),
          updated_at: restaurant.updated_at.to_string()
        }
      }).collect();
      let reply = GetRestaurantsResponse { restaurants };
      Ok(Response::new(reply))
    }


    async fn get_restaurant(
        &self,
        request: Request<GetRestaurantRequest>,
    ) -> ServiceResult<Response<GetRestaurantResponse>> {
      let id = &request.get_ref().id;
      let restaurant = restaurant::Restaurant::get(&self.pool, id).await;

      if restaurant.is_err() {
        return Err(Status::new(tonic::Code::NotFound, "Restaurant does not exist"))
      }

      let restaurant = restaurant.unwrap();
      let reply = GetRestaurantResponse {
          id: restaurant.id.to_string(),
          name: restaurant.name,
          address: restaurant.address,
          email: restaurant.email,
          phone: restaurant.phone,
          created_at: restaurant.created_at.to_string(),
          updated_at: restaurant.updated_at.to_string()
      };
      Ok(Response::new(reply))
    }

    async fn create_restaurant(
      &self,
      request: Request<CreateRestaurantRequest>
    ) -> ServiceResult<Response<CreateRestaurantResponse>> {
      let details = request.get_ref();
      let new_restaurant = NewRestaurant {
        name: details.name.clone(),
        address: details.address.clone(),
        email: details.email.clone(),
        phone: details.phone.clone()
      };

      let restaurant = restaurant::Restaurant::create(&self.pool, new_restaurant).await;

      if restaurant.is_err() {
        return Err(Status::new(tonic::Code::Internal, "Failed to create restaurant"))
      }

      let restaurant = restaurant.unwrap();
      let reply = CreateRestaurantResponse {
        restaurant: Some(GetRestaurantResponse {
          id: restaurant.id.to_string(),
          name: restaurant.name,
          address: restaurant.address,
          email: restaurant.email,
          phone: restaurant.phone,
          created_at: restaurant.created_at.to_string(),
          updated_at: restaurant.updated_at.to_string()
        })
      };

      Ok(Response::new(reply))
    }

    async fn delete_restaurant(
      &self,
      request: Request<DeleteRestaurantRequest>
    ) -> ServiceResult<Response<DeleteRestaurantResponse>> {
      let id = request.get_ref().id.clone();

      let deleted = restaurant::Restaurant::destroy(&self.pool, Uuid::parse_str(&id).unwrap()).await;

      if deleted.is_err() {
        return Err(Status::new(tonic::Code::Internal, "Failed to delete restaurant"))
      }

      Ok(Response::new(DeleteRestaurantResponse{}))
    }
}