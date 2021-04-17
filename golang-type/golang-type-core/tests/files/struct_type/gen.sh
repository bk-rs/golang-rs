#!/usr/bin/env bash

# ./tests/files/struct_type/gen.sh

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

categories=('embedded_field' 'normal' 'tag')

for category in "${categories[@]}"
do
go run "${script_path_root}${category}.go"

cat "${script_path_root}${category}.go" | sed -n '/^[ \t]*var _ struct {/,/^[ \t]*}/p' | sed 's/^[ \t]//g; s/[ \t]$//g; s/^[ \t]*var _ //; /^$/d;' > "${script_path_root}${category}.txt"
done
