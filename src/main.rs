#[tokio::main]
async fn main() {
	use filters::*;
    let routes = color();

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

mod filters{
    use warp::Filter;
    use super::handlers;

    pub fn color() ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{ 
        warp::get()
            .and_then(handlers::handle_color)
    }
}

mod handlers{
    use warp::http::StatusCode;
    use std::convert::Infallible;

    pub async fn handle_color() -> Result<impl warp::Reply, Infallible> {
        // "Alright, alright, alright", Matthew said.
        Ok(warp::reply::with_status("{\"color\": \"red\"}", StatusCode::OK))
    }
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;
    use super::filters;
    use std::str;

    #[tokio::test]
    async fn try_color() {
        let api = filters::color();

        let response = request()
            .method("GET")
            .path("/")
            .reply(&api)
            .await;

			assert_eq!(response.status(), StatusCode::OK);

			let result: Vec<u8> = response.into_body().into_iter().collect();
			let result = str::from_utf8(&result).unwrap();
			assert_eq!(result, "{\"color\": \"red\"}");
    }
}
