#!/usr/bin/env bash

set -e


if [ $# -eq 0 ]; then
    echo "Pass 'serve', 'build' or 'build-watch' as arguments"
    exit 1
fi

if [[ -z "${YEW_PUBLIC_PATH}" ]]; then
    echo "Setting YEW_PUBLIC_PATH to '/yew/"
    export YEW_PUBLIC_PATH="/yew/"
else
    echo "Variable YEW_PUBLIC_PATH is set to ${YEW_PUBLIC_PATH}"
fi

cd yew
yarn build
cd -

cd elm
yarn build
cd -

case $1 in
   "serve")
   yarn start
   ;;
   "build")
   yarn build
   ;;
   "build-watch")
   yarn build && serve dist -l 10000
   ;;
esac
