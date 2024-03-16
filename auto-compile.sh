#!/usr/bin/bash
while true; do
  # wait for changes in resume.tex
  _=$(watch -d -t -g -n 0.1 ls -lR --time-style=full-iso ./resume.tex)
  
  clear
  time pdflatex resume.tex
done