#!/usr/bin/env bash

export network=$3
export name="spore-dob-0"

if [ -z $network ]
then
    export network="testnet"
fi

echo "deploying $name from $2 to $1 on $network"

ckb-cli --url $1 deploy gen-txs --from-address $2 --fee-rate 1000 --deployment-config ./deployment.toml \
    --info-file ./$name.json --migration-dir ./migration/$network --sign-now

echo "ckb transacion file '$name.json' has generated"

ckb-cli --url $1 deploy apply-txs --info-file ./$name.json --migration-dir ./migration/$network

rm ./$name.json

echo "deployment finished"

# usage: ./deploy.sh https://testnet.ckbapp.dev/ ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqfqmf4hphl9jkrw3934mwe6m3a2nx88rzgdlw820
