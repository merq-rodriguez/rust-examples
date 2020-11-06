# Bibliotecas

* `warp` para crear la API
* `Tokio` para ejecutar un servidor asincrónico
* `Serde` para ayudar a serializar JSON entrante
* `parking_lot` para crear un ReadWriteLock para su almacenamiento local

Los `Filters` son una forma de analizar una solicitud y compararla con una ruta que creamos. Entonces, cuando inicia el servidor a través cargo run y apunta su navegador a `localhost:3030/hello/WHATEVER`, warp envía esta solicitud a través de sus filtros y ejecuta la primera que se activa.



