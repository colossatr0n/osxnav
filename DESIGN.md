A scratchpad for now, thinking about the general design.

One open question: Should osxnav change it's name and also support linux in the
future? (Could replace keynav.) Whatever the answer is, I'll try to keep the
config file design cross platform.

There should be a config file similar to keynav. Left side is key code/name,
right side is command. Here is my keynav config, translated into this nav
program:

```
c: reset

e:       end
esc:     end
shift+e: end

h: cut-left
j: cut-down
k: cut-up
l: cut-right

shift+h: move-left
shift+j: move-down
shift+k: move-up
shift+l: move-right

semicolon warp,end

super+h cut-left
super+j cut-down
super+k cut-up
super+l cut-right

q record
g playback

b history-back

w warp

p warp,click 4
n warp,click 5

shift+p warp,click 4 shift
shift+n warp,click 5 shift

space warp,click-1
m     warp,click-2
r     warp,click-3

super+space warp,drag-1
super+m     warp,drag-2
super+r     warp,drag-3

shift+space warp,drag 1
shift+m     warp,drag 2
shift+r     warp,drag 3

ctrl+space warp,drag 1 ctrl
ctrl+m     warp,drag 2 ctrl
ctrl+r     warp,drag 3 ctrl
```




EX:

```
h: cut-right
l: cut-left
j: cut-down
k: cut-up
c: reset
esc: exit
```

Commas are also supported:

```
space: click,exit
```

