use axum::{
    extract::Query,
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use std::collections::HashMap;

// User model
#[derive(Debug, Serialize)]
struct User {
    /// Unique user identifier
    id: u32,
    /// Full name of the user
    name: String,
    /// Email address of the user
    email: String,
    /// Age of the user in years
    age: u32,
}

// Query parameters for filtering/pagination
#[derive(Debug, Deserialize)]
struct UserQuery {
    /// Maximum number of users to return
    #[serde(default)]
    limit: Option<usize>,
    /// Filter users by name (case-insensitive partial match)
    #[serde(default)]
    name: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create the router with routes and API docs
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users))
        .route("/docs", get(swagger_ui))
        .route("/openapi.json", get(openapi_spec));

    // Start the server
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("📋 Available endpoints:");
    println!("  GET /         - Welcome message");
    println!("  GET /users    - Get all users");
    println!("  GET /users?limit=5          - Limit results");
    println!("  GET /users?name=john        - Filter by name");
    println!("📚 API Documentation:");
    println!("  GET /docs     - Swagger UI documentation");
    println!("  GET /openapi.json - OpenAPI specification");

    axum::serve(listener, app).await.unwrap();
}

// Root endpoint
async fn root() -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();
    response.insert("message".to_string(), "Welcome to Simple Users API".to_string());
    response.insert("version".to_string(), "1.0.0".to_string());
    Json(response)
}

// Get users endpoint with optional filtering
async fn get_users(Query(params): Query<UserQuery>) -> Json<Vec<User>> {
    // Sample data (in real app, this would come from a database)
    let users = vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            age: 30,
        },
        User {
            id: 2,
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            age: 25,
        },
        User {
            id: 3,
            name: "Bob Johnson".to_string(),
            email: "bob@example.com".to_string(),
            age: 35,
        },
        User {
            id: 4,
            name: "Alice Brown".to_string(),
            email: "alice@example.com".to_string(),
            age: 28,
        },
        User {
            id: 5,
            name: "Charlie Wilson".to_string(),
            email: "charlie@example.com".to_string(),
            age: 42,
        },
    ];

    let mut filtered_users = users;

    // Filter by name if provided
    if let Some(name_filter) = params.name {
        filtered_users.retain(|user| {
            user.name.to_lowercase().contains(&name_filter.to_lowercase())
        });
    }

    // Apply limit if provided
    if let Some(limit) = params.limit {
        filtered_users.truncate(limit);
    }

    Json(filtered_users)
}

// Swagger UI endpoint
async fn swagger_ui() -> Html<String> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="description" content="SwaggerUI" />
    <title>Simple Users API Documentation</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
</head>
<body>
<div id="swagger-ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js" crossorigin></script>
<script>
  window.onload = () => {
    window.ui = SwaggerUIBundle({
      url: '/openapi.json',
      dom_id: '#swagger-ui',
      presets: [
        SwaggerUIBundle.presets.apis,
        SwaggerUIBundle.presets.standalone,
      ],
      layout: "BaseLayout",
      deepLinking: true,
      showExtensions: true,
      showCommonExtensions: true
    });
  };
</script>
</body>
</html>
    "#.to_string())
}

// OpenAPI specification endpoint
async fn openapi_spec() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Simple Users API",
            "description": "A simple REST API for managing users",
            "version": "1.0.0",
            "contact": {
                "name": "API Support",
                "email": "support@example.com"
            }
        },
        "servers": [
            {
                "url": "http://127.0.0.1:3000",
                "description": "Development server"
            }
        ],
        "paths": {
            "/": {
                "get": {
                    "tags": ["general"],
                    "summary": "Welcome message",
                    "description": "Returns a welcome message and API version",
                    "responses": {
                        "200": {
                            "description": "Welcome message",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "message": {
                                                "type": "string",
                                                "example": "Welcome to Simple Users API"
                                            },
                                            "version": {
                                                "type": "string",
                                                "example": "1.0.0"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/users": {
                "get": {
                    "tags": ["users"],
                    "summary": "Get all users",
                    "description": "Retrieve a list of users with optional filtering and pagination",
                    "parameters": [
                        {
                            "name": "limit",
                            "in": "query",
                            "description": "Maximum number of users to return",
                            "required": false,
                            "schema": {
                                "type": "integer",
                                "minimum": 1,
                                "maximum": 100,
                                "example": 10
                            }
                        },
                        {
                            "name": "name",
                            "in": "query",
                            "description": "Filter users by name (case-insensitive partial match)",
                            "required": false,
                            "schema": {
                                "type": "string",
                                "example": "john"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of users",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/components/schemas/User"
                                        }
                                    },
                                    "examples": {
                                        "users": {
                                            "summary": "Example users",
                                            "value": [
                                                {
                                                    "id": 1,
                                                    "name": "John Doe",
                                                    "email": "john@example.com",
                                                    "age": 30
                                                },
                                                {
                                                    "id": 2,
                                                    "name": "Jane Smith",
                                                    "email": "jane@example.com",
                                                    "age": 25
                                                }
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "User": {
                    "type": "object",
                    "required": ["id", "name", "email", "age"],
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "Unique user identifier",
                            "example": 1
                        },
                        "name": {
                            "type": "string",
                            "description": "Full name of the user",
                            "example": "John Doe"
                        },
                        "email": {
                            "type": "string",
                            "format": "email",
                            "description": "Email address of the user",
                            "example": "john@example.com"
                        },
                        "age": {
                            "type": "integer",
                            "minimum": 0,
                            "maximum": 150,
                            "description": "Age of the user in years",
                            "example": 30
                        }
                    }
                }
            }
        },
        "tags": [
            {
                "name": "general",
                "description": "General API endpoints"
            },
            {
                "name": "users",
                "description": "User management endpoints"
            }
        ]
    }))
}

// Cargo.toml:
/*
[package]
name = "simple-users-api"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
*/