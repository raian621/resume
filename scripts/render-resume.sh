#!/bin/sh

if [ -z "$USER_UID" ]; then
  USER_UID=$(id -u)
fi
if [ -z "$USER_GID" ]; then
  USER_GID=$(id -g)
fi

mkdir -p /tmp/output
for resume_filepath in data/*.yml; do
  echo $resume_filepath
  resume_tex_filepath=$(basename "$resume_filepath" | sed -E 's/^(.+)\.yml$/\/tmp\/output\/\1.tex/')
  echo $resume_tex_filepath
  ./target/release/resume -i "$resume_filepath" -o "$resume_tex_filepath"
  pdflatex -output-directory=/tmp/output "$resume_tex_filepath"
done

cp /tmp/output/*.pdf out
chown -R $USER_UID:$USER_GID out
