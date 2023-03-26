# Rust RAT with Real time communications

- **Agent**
- **C&C**
- **Client**

## For Real time communication 
- Short Polling
- WebSockets 
- Server Sent Events SSE
- Long Polling

Long polling is extremly efficient in Rust, thanks to async, very few resources are used per connection while other langs use OS thread

### for API we can choose
- gotham
- axum 
- actix
- rocket 
- hyper
- **warp**
- tide 

#### Choosing HTTP Library
- hyper
- reqwest
- awc
- **ureq**
- surf

### For DB we can choose.. 
- diesel
- tokio-postgres
- **sqlx**


## Presentation Layer aka THE API!!!!!!!
-----------------------------------------
The presentation layer JSON API is going to handle very important task for use.
Now you are going to learn in rust lang how to do 
- Routing
- Decoding Requests
- calling Service layer
- Encoding Responses


**Routing is process of mathcin an HTTP request to correct function**

### in Server dir
- Presentation Layer
- Service Layer
- Repository Layer 


- By Mohammed Maali for cyber security experts
