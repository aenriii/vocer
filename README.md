# vocer

a simple, fast, and stylish VRChat OSC application

### simple

just press the buttons and you'll eventually get there! start off with a template
that'll let you attach what you'd like and quickly get back to ~~whatever gay thing
you're doing~~ playing VRChat or start from scratch, turn on and tune pieces to
your liking!

### fast

this application is built in rust (obligatory blazing fast! :rocket:),
which allows it to function properly while using few system resources. that, in
addition with custom libraries fine-tuned to do exactly what we need them to
do (even if that means spec-noncompliant) which removes a lot of the BS we don't
need.

in addition, if you already have a configuration set up to your liking you can
run the app without a gui to save even more performance! just add
`vocer --profile PROFILE_NAME` to your VRChat launch commands!

### stylish

this application is built with the Slint UI library, using a custom widget pack
and color pallete based on the catppuccin style guide!

## features

### local time

show your local time to those around you!

### spotify integration (with lyrics!)

bring your own spotify application keys *or* use ours! log in to spotify and
show others what you're listening to! in addition, you can use synced lyrics
(BYO api) to show the lyrics synced up to your listening! this also includes
togglable romanization for some non-latin languages!

### windows media session shower

spotify not your thing? enable this module and the now playing media session
broadcasted by windows will be used instead of a spotify now-playing status!
do note that trying to get lyrics for these is unreliable at best, and its
recommended to only enable that if you know what youre listening to is on
spotify.

### static and cycling text

show your words! include lines of either static or cycling text in your chatbox.


## why are you putting this much effort into this?

im using this project as an opportunity to learn slint! ive found the ui library
both fascinating and promising since i first heard of it but i havent had the
opportunity to actually try it out until now. 

---

 <p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><a property="dct:title" rel="cc:attributionURL" href="https://github.com/aenriii/vocer">vocer</a> by <a rel="cc:attributionURL dct:creator" property="cc:attributionName" href="https://aenri.loveh.art">Aenri Rose Lovehart</a> is marked with <a href="https://creativecommons.org/publicdomain/zero/1.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC0 1.0<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/zero.svg?ref=chooser-v1" alt=""></a><br><small>however, if you'd like to attribute me in whatever you make, that'd be greatly appreciated &lt;3 </small></p>