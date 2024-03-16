#!/usr/bin/bash
while true; do
  watch -d -t -g ls -lR ./resume.tex && pdflatex resume.tex
done