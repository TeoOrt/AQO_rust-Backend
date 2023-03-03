//ENUMS are types that are limited
//for example instead of passing a string for  GETt
//you can use the GET enum for better handling of http request

    pub enum Method{
        GET,
        POST,
        PUT,
        HEAD,
        CONNECT,
        OPTIONS,
        TRACE,
        PATCH,
        DELETE,
    }
/*
GET /user?id=10  HTTP/1.1\r\n
HEADER \r\n
BODY    
*/