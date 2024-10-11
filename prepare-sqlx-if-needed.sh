#!/bin/bash

cargo sqlx prepare --all -- --all-targets --all-features
git add .sqlx/*
exit_code=$?

exit $exit_code
