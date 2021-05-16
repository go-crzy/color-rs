
// use warp::Filter;

// pub fn color() -> impl Filter<Extract = (&'static str,)> + Copy {
// 	warp::any().map(|| "{\"color\": \"red\"}")
// }	

// #[cfg(test)]
// mod tests {
// 	use super::*; 

// 	#[test]
//     fn test_add() {
// 		let filter = color();

// 		let value = warp::test::request()
// 			.path("/")
// 			.reply(&filter);

// 		assert_eq!(value.status(), 200);
// 	}
// }

mod filters{
    use warp::Filter;
    use super::handlers;

    pub fn list() ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{ 
        warp::path!("holodeck")
            .and(warp::get())
            .and_then(handlers::handle_list)
    }
}

mod handlers{
    use warp::http::StatusCode;
    use std::convert::Infallible;

    pub async fn handle_list() -> Result<impl warp::Reply, Infallible> {
        // "Alright, alright, alright", Matthew said.
        Ok(StatusCode::OK)
    }
}

#[tokio::main]
async fn main() {
	use filters::*;
    let routes = list();

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}


#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;
    use super::filters;

    #[tokio::test]
    async fn try_list() {
        let api = filters::list();

        let response = request()
            .method("GET")
            .path("/holodeck")
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }
}
