use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub mod schema;

type LuxSchema = Schema<schema::Query, EmptyMutation, EmptySubscription>;

// TODO: Serve Apollo Studio instead of GraphiQL
const _APOLLO_STUDIO: &'static str = "<!DOCTYPE html>\
<html>\
  <head>\
    <title>Sandbox</title>\
  </head>\
  <body>\
    <div id=\"sandbox\" style=\"position: absolute; top: 0; right: 0; bottom: 0; left: 0\"></div>\
    <script src=\"https://embeddable-sandbox.cdn.apollographql.com/_latest/embeddable-sandbox.umd.production.min.js\"></script>\
    <script>\
      new window.EmbeddedSandbox({\
        target: \"#sandbox\",\
        initialEndpoint: window.location.href,\
      });\
    </script>\
  </body>\
</html>";

async fn execute_graphql(schema: web::Data<LuxSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn serve_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(_APOLLO_STUDIO.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Lux GraphQL server at 0.0.0.0:4000...");

    let lux_schema: LuxSchema =
        Schema::build(schema::Query, EmptyMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(lux_schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(execute_graphql),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(serve_graphiql),
            )
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
