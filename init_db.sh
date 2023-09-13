#!/bin/bash
#
echo "creating database user"
docker exec -u postgres -ti devorum-db-1 createuser -P devorum
echo "creating database"
docker exec -u postgres -ti devorum-db-1 createdb --E utf8 -O devorum devorum
