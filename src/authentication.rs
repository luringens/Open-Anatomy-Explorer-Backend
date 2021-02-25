use std::convert::TryFrom;

use crate::MainDbConn;
use crate::{models, schema::users::dsl as users};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    outcome::IntoOutcome,
    request::{self, FromRequest, Outcome, Request},
};

pub struct User(pub models::User);

impl<'a, 'r> FromRequest<'a, 'r> for &'a User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<&'a User, !> {
        // This closure will execute at most once per request, regardless of
        // the number of times the `User` guard is executed.

        let user_result = request.local_cache(|| {
            request.guard::<MainDbConn>().succeeded().and_then(|db| {
                request
                    .cookies()
                    .get_private("user_id")
                    .and_then(|cookie| cookie.value().parse().ok())
                    .and_then(|id: i32| {
                        users::users
                            .filter(users::id.eq(&id))
                            .load::<models::User>(&*db)
                            .ok()
                            .and_then(|mut o| o.pop())
                    })
                    .map(User)
            })
        });
        user_result.as_ref().or_forward(())
    }
}

pub struct Moderator(pub models::User);

impl<'a, 'r> FromRequest<'a, 'r> for Moderator {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Moderator, !> {
        let user = request.guard::<&User>()?;

        match models::Privilege::try_from(user.0.privilege) {
            Ok(models::Privilege::Moderator) | Ok(models::Privilege::Administrator) => {
                Outcome::Success(Moderator(user.0.clone()))
            }
            _ => Outcome::Forward(()),
        }
    }
}

pub struct Admin(pub models::User);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, !> {
        let user = request.guard::<&User>()?;

        if user.0.privilege == models::Privilege::Administrator as i32 {
            Outcome::Success(Admin(user.0.clone()))
        } else {
            Outcome::Forward(())
        }
    }
}
