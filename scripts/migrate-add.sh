#!/usr/bin/env bash

export DATABASE_URL='postgres://development:development@localhost:5432/development'

cd .. || exit 1

sqlx migrate add create_tables
