#!/bin/sh

if [ $# -ne 1 ]; then
  echo "usage: ./open.sh  [task_name]" 1
  exit 1
fi
open "https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_$1"
