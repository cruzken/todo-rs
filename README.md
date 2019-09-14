# todo-rs

A todo app written in Rust, using Actix, Diesel and SQLite. Frontend uses Vue, Axios, and Bulma. A live demonstration can be seen [here.](http://todo.kencruz.ca)

## Getting the image:
The image has been pushed to [Dockerhub](https://hub.docker.com/r/cruzken/todo/)  
So you can just:

```
docker pull cruzken/todo
```
## Running the image:
- Run the Docker container: `docker run -p 8088:8088 --rm --name todo cruzken/todo`
- Goto [http://localhost:8088](http://localhost:8088)

## Building the image from source.:

- First, clone this project.

```
$ git clone https://github.com/cruzken/todo-rs.git
```

- Second, change directory to the just now cloned repository.

```
$ docker build -t cruzken/todo .
```

