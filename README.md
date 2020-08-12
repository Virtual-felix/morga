# Phoenix

All you can see for now is ash and light distorted by the heat.


## Development
#### API
```bash
cd api
cargo build
```
or (with docker)
```bash
docker build -t ph-api:dev .
```

then to run
```bash
cargo run
```
or (with docker)
```bash
docker run -it --init --rm \
-p 3001:3001 \
-e ROCKET_PORT=3001 -e ROCKET_ADDRESS=0.0.0.0 \
-v ${PWD}/src:/usr/src/api/src \
ph-api:dev
```

### APP
```bash
cd app
```

```
npm install
```
or (with docker)
```bash
docker build -t ph-app:dev .
```

then to run

```bash
npm start
```
or (with docker)
```bash
docker run -it --rm \
-v ${PWD}:/app \
-p 3000:3000 \
-e CHOKIDAR_USEPOLLING=true \
ph-app:dev
```

### Docker compose (recommanded)
You can also build and run everything at once with docker compose
```bash
docker-compose build
```
```bash
docker-compose up
```


## Production

In production, using docker compose is highly recommanded.
```bash
docker-compose -f docker-compose.prod.yml build
```
```bash
docker-compose -f docker-compose.prod.yml up
```
The app will be accessible on port 80.

More option will be available later to be able to tweak every possible thing and fully respect the [twelve factors](https://12factor.net/).
