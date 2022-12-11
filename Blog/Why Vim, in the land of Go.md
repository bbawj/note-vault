---
title: "Why Vim, in the land of Go"
date: 2022-12-10
lastmod: 2022-12-11
---
Editor wars have been fought from at least [1985](https://en.wikipedia.org/wiki/Editor_war). In 2022 however, the [majority](https://survey.stackoverflow.co/2022/#most-popular-technologies-new-collab-tools) continue to trend towards Visual Studio Code as their preferred choice of an integrated developer environment (IDE). At my workplace where the primary language is Go, the editor on screens, amongst the different colour schemes and themes can be quite easily recognised as JetBrain's proprietary GoLand. Being a paid piece of software, it of course includes most of the feature set which one might expect from a modern IDE: syntax highlighting, symbol navigation methods, built-in debugger etc. Even more importantly, it seemed like GoLand users made bigger bucks than Vim users[^1]!
![](https://i.imgur.com/Rz6WHs3.png)
At the behest of one colleague, who felt that GoLand was slow and memory intensive, I decided to come up with my case for why I use vim in the land of Go.
## Vim Motions...Sickness
Vim is not *just* vim. Vim comprises of its interface (the program running in a terminal window) and vim **motions**. These are the keyboard shortcuts that can be tied to cursor movements, file operations and even custom functions. I won't go into the weeds about the different commands and modes available as there are a ton of interactive (and even gamified, if that's your type of thing) tutorials out there which do a much better job of what I can hope to do here. Here are some:
- vimtutor
- [Learn-Vim](https://github.com/iggredible/Learn-Vim)
- [vim-be-good](https://github.com/ThePrimeagen/vim-be-good)

But why should you learn a bunch of new commands and keystrokes? It seems like a massive time sink. Everyday, some new technology comes out demanding your attention and this just ain't helping. 
### Need for speed
This brings me to my first reason, it makes you really *really* fast. The basic commands keep your hands on the home row of the keyboard, allowing you to just churn out lines and lines of text without ever touching the mouse. And of course, as programmers, we are not always writing code. Time is spent thinking and designing what is the *best* way to solve a problem. But during this deliberation process, we like to try things, add a few lines here and refactor a function there. In general, we like to break stuff to understand how things work and what we should do next. Being able to break stuff quickly, reducing mouse distractions, speeds up the iterative process that is software engineering.
### SSS combo
[Speed is fun](https://www.scienceabc.com/pure-sciences/why-do-we-feel-so-thrilled-by-speed.html). But if you have ever played brawler or hack and slash type games, you know the thrill of hitting insane combos. This is the same way I feel about vim motions. Want to refactor a nested function? `:10<CR>V10jd<C-d>P`. Want to change all its arguments? `ci(`. Want to give up? `:qa!<CR>`. Keeping that flow state, moving fast, pushing out combos just adds up to quite a lot of fun.
![](Pics/Pasted%20image%2020221210223916.png)
## Vim the program
Now, let's talk about vim the program itself. In terms of applicability, vim is still *somewhat* immemorial. Vi is installed by default in various Linux distributions, allowing you to interface with server files proficiently. But who cares? You just want to be able to write and debug Go code, see pretty rainbow bracket colours and run a separate terminal program all in the same view. Vim *can't* do that, after all, its website looks like it was made in the 90s:
![](https://i.imgur.com/ml1lN4w.png)

![](https://i.imgur.com/utfZYEn.png)
### Running naked makes you faster?
Vim starts of relatively barebones. From its website you can see the key features listed being a multi level undo tree and powerful search and replace, which are all things you would already expect to have. This means that at the start, writing code in vim is extremely painful. It will feel like you are writing code on Notepad but with the added difficulty of hundreds of commands. However, this also means that it is fast. It launches instantaneously on the terminal window and even on low performance virtual machines, the experience is still decent. 

But what is the point of it all if you are just going to be worse off as a programmer? Vim has an extensive plugin system. Adding functionality which you want is simple, and usually involves finding existing plugins on GitHub, adding them to your initialisation file and configuring the options that you want. See the key here is what *you* want. You get to decide what features you wish to add to your editor, and what you consider bloat. **To quickly get all the necessary features you are used to in GoLand into Vim, there is [this plugin](https://github.com/fatih/vim-go).**
### Nah it just allows you to tinkle I mean tinker while you run
All this control does not just limit you to the functionality of things. Looking at the screen for hours a day means aesthetics is just as important to the programmer. Vim allows you extensive options to make it look the way you want to. This type of persistent tinkering may put off some of you who simply wish for some sensible defaults out of the box and at the beginning, it will be constant tinkering to get something you like. However, I assure you that the satisfaction of having something completely personalised to your taste and style will make coding in it 10x more enjoyable.

My setup for Go in this year's Advent Of Code:![](https://i.imgur.com/dQBN1lq.png)
### How about running in the open?
Vim is open source. This means you can (if you want) scrutinise the code for any malicious intent. This does not automatically make Vim *better* per-se. Instead, this might mean that stability is not as guaranteed as compared to an editor where people are paying $69.99 per month for. In fact, Paul Lutus would request for the user to "stop complaining for a while and make the world a better place."[^2]

Personally, I use [NeoVim](https://github.com/neovim/neovim), which is a fork off Vim that encourages community contributions among other things. For essential Go development features, checkout the wonderful [go.nvim](https://github.com/ray-x/go.nvim) plugin.
## Concrete steps out of the tarpit
Past all my blabbering, I wish to offer some actionable steps which you can take.
1. I would argue that learning vim motions bring you 80% of the way towards using vim as your daily driver. Hence, don't start with Vim, start with vim motions. Look for  options or plugins that enable the use of vim key bindings. For GoLand users, look to [IdeaVim](https://plugins.jetbrains.com/plugin/164-ideavim). This means that you can get good with the essential vim commands without leaving the comfort of GoLand.
2. When you feel ready to leave the nest, I recommend installing NeoVim and fiddling with the configurations. Here I also recommend looking at videos, which can offer step by step walkthroughs on the the general ideas behind configuration. 
- [Your first vimrc](https://www.youtube.com/watch?v=x2QJYq4IX6M)
- [Neovim from scratch](https://www.youtube.com/watch?v=ctH-a-1eUME)
As with anything that is configurable, one must accept the inevitable situation of things breaking. It helps to treat these situations like mini side projects, ones that will help solidify your understanding of the tools you use and make you a better developer out of it.
3. Get inspired. There is a great community out there that use Vim to make incredible things. Many make plugins which I cannot live without, and many others create insanely "riced" personal development environments out of their editor. It's hard to get into something tough, if you can't see or want the end goal. [Reddit](https://www.reddit.com/r/neovim)is a good place to explore what you may be interested in. I can also highly recommend following some extremely knowledgeable and entertaining vim content creators like [ThePrimeagen](https://www.youtube.com/@ThePrimeagen) and [TJ](https://www.youtube.com/channel/UCd3dNckv1Za2coSaHGHl5aA).
## References
[^1]: https://survey.stackoverflow.co/2022/#top-paying-technologies-integrated-development-environment
[^2]: https://arachnoid.com/careware/index.html