[![docker-build](https://github.com/weidingerhp/surreal_db_awattar_prices/actions/workflows/publish-dockerhub.yml/badge.svg)](https://github.com/weidingerhp/surreal_db_awattar_prices/actions/workflows/publish-dockerhub.yml)

## A container to fetch aWATTar prices

A small rust program that takes the prices from the aWATTar API and stores them in a database. This build creates a docker container that can be used to run the program.

### Usage

```docker run hapewe/awattar_price_fetch:latest```

to specify the instance of the database to use, set the environment variables 
- **SURREALDB_URL** DB-Url. The default is `localhost:8000`
- **SURREALDB_USER** DB-User. The default is `root`
- **SURREALDB_PASS** DB-Password. The default is `root`
- **DEBUG_READ_DATA** Dumps data from awattar. The default is `false`

```docker run -e SURREALDB_URL=my_surreal_host:8000 -e SURREALDB_USER=root -e SURREALDB_PASS=rootpass hapewe/awattar_price_fetch:latest```