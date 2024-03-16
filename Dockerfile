FROM ubuntu:latest

# avoid hanging on tz-data latex dependency installation >:|
RUN ln -snf /usr/share/zoneinfo/America/Chicago /etc/localtime && echo America/Chicago > /etc/timezone
RUN apt-get update && apt install texlive-latex-base texlive-fonts-recommended texlive-fonts-extra texlive-latex-extra -y
