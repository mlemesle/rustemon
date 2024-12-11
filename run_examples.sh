#!/bin/bash

run_example() {
  additional_features=""
  case $1 in
    *_serialize)
      additional_features="--features serialize"
    ;;
  esac
  
  if cargo run --example $1 $additional_features &> /dev/null; then
    echo -e "\e[32m ${1} ok.\e[0m";
  else
    echo -e "\e[31m ${1} error.\e[0m";
  fi
}

for file in ./examples/*
  do
    name=$(basename $file)
    base=${name%.rs}
    run_example $base
done
