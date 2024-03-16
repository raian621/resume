# resume
LaTeX template for my personal resume

I've only tinkered with this on Ubuntu, so that's the only platform this project is currently intended for.

Based off of [sb2nov/resume](https://github.com/sb2nov/resume/)

Forked from [Jake Gutierrez's Resume](https://github.com/jakegut/resume) (thanks Jake)

## Dependencies
On Ubuntu, you need these packages in order to compile this LaTeX resume:
- `texlive-latex-base`
- `texlive-fonts-recommended`
- `texlive-fonts-extra`
- `texlive-latex-extra`

So running

```sh
sudo apt-get install texlive-latex-base texlive-fonts-recommended texlive-fonts-extra texlive-latex-extra
```

should get you up and running on your local machine.

## Development

I used Visual Studio Code with the [`vscode-pdf`](https://marketplace.visualstudio.com/items?itemName=tomoki1207.pdf) extension installed so I could view the generated PDF while working on the LaTeX resume. 

To compile the PDF from the LaTeX resume, run

```sh
pdflatex resume.tex
```

but you can also use the `auto-compile.sh` convenience script to automatically recompile the PDF every time you make a change to `resume.tex`

```sh
./auto-compile.sh
```

## Continuous Delivery
In order for the GitHub Actions workflows to work properly, you must set up [Github Actions permissions](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository) for this repo to allow Actions to create packages and releases.

It's kind of silly, but every time you push to this GitHub repo, a GitHub Actions workflow is triggered that creates a new release containing your resume in PDF format. The release name is set to whatever the branch you pushed was called, so pushing to a `fullstack` branch will create a `fullstack` resume release, pushing to a `burger-flipper` branch will create a `burger-flipper` resume release, etc..
