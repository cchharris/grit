# grit
GRIT - A rust re-implementation of git. 


# Why does grit exist?
After several years as a software developer on a large, long-lived desktop application, I found myself falling behind on modern skills and knowledge.  As a C++ developer, I am interested in [Rust](https://www.rust-lang.org/) as a language due to its promises of speed, security, and efficiency.  It is also consistently rated [the most loved](https://insights.stackoverflow.com/survey/2021#technology-most-loved-dreaded-and-wanted) language on Stack Overflow.

At work, we unfortunately still do not use git.  I use it on my own time for my own projects, but one person committing and branching does not cause the scenarios I expect to see at my job once we fully commit to hundreds of developers in the system.  I also don't fully understand the capabilities git offers.  What better way to learn what some software is capable of doing, than by reimplementing all those features yourself?

# Philosophy
The goal of the code of this project is to produce a software suite that fulfills the git command line experience.  Users should be able to, from within a compatible git-like repository, replace most `git` commands with `grit`, and it should _just work_.  At this point in time I am just one person.  I make no guarantees to the safety of this software - use at your own peril.  Git has had a multitude of security attacks against it, as I am replicating the source code to the best of my abilities, I will likely be implementing the patches to those attacks as well, but again, no guarantees.  Also, no guarantees that I do not introduce my own vulnerabilities by mistake.

# Getting Started
I am selecting [git v2.35.3](https://github.com/git/git/tree/v2.35.3), for an important reason.  It was the latest published release before I wrote this page.  Similarly, I am using [rust v1.60.0](https://github.com/rust-lang/rust/releases/tag/1.60.0), for the exact same reason.  I expect the rust compiler version will change throughout the project - more due to me idly running `rustup update` than feature hunting.  The git version is much more rigid - I don't want to have a moving target to code towards.  I might target pulling in specific security vulnerabilities, should I ever get to the point they would exist in the code base, but that's a future conversation.

I don't yet know how I'm going to approach this as a whole.  This will be a learning experience for us all.

# Name
As described at [the git repository on github](https://github.com/git/git), git's name varies depending on your mood.  Grit follows the same naming philosophy - whatever fits your current mood.  As I write this, grit matches the definition given to it by [Merriam-Webster](https://www.merriam-webster.com/dictionary/grit):

> firmness of mind or spirit : unyielding courage in the face of hardship or danger

As I set out on this experiment, I know this won't be a quick or easy project.  But I know ultimately it can be a fulfilling one.  And who knows what it might become in the future?

Also `grit` sounds better than `rgit`, and I wanted to get an `r` in the name.
