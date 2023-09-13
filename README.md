# Devorun

for discussions that matter

# nntpisnotdead

Threaded forum, aiming to revive goog old days of NNTP.

# Requirements

I am running this with docker 24 and docker-compose-v2 installed, anything more recent should also work.

# Setup

First, we will need to start required docker containers:

docker compose -p devorum up

(-p devorum matters here, otherwise you may need to adjust container names in further commands)

We will need to create database user and the database:

./init_db.sh

You will be asked for password for new user.
