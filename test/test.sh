#!/bin/bash

CASE="$(cd $(dirname $0);pwd)/case"

assert() {
  input_path="${1}"

  line=$(cat "${input_path}" | wc -l)

  sed -n 1,`expr ${line} - 1`P "${input_path}" > tmp.c
  expected="$(sed -n ${line}P ${input_path})"

  ./ecc tmp.c tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "${actual}" = "${expected}" ]; then
    echo "${input_path} => ${actual}"
  else
    echo "${input_path} => ${expected} expected, but got ${actual}"
    exit 1
  fi
}

for f in `ls ${CASE}/*`; do
  assert $f
done

echo OK
