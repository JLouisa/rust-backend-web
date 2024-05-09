# Start Redis

$ docker run --name my-redis -p 6379:6379 -d redis

## See all Docker containers

$ docker ps -a

## Stop Redis Container

$ docker stop [container_id_or_name]

## Remove Redis image

$ docker rm [container_id_or_name]
