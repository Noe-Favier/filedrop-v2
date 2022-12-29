#!/bin/sh -x

base_string="you'll have to replace that value"

if [ `grep -rnw ./Rocket.toml -e "${base_string}" | wc -l` -ne 0 ]
then 
    sed -i "s|${base_string}|`openssl rand -base64 32`|g" Rocket.toml
fi

exec ./filedrop-v2