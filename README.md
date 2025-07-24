My SpeccyTrain project uses RLE chars/udgs/attrs to set up the scene. Reasonably efficient as an initial optimization, but a pain in the ass to make changes to.

So I need a way to turn a bitmap version into an RLE version.

Obviously there are lots of tools out there I could use and with some fiddling make my life easier. 

Or I could write a little tool to do exxactly what I need: char/udg/attr lists into assembler blocks of code.
But that would be kinda boring coding...

Unless... Unless I took this as an opportunity to learn something else new.

Rust.

After having to fight Java's bloody garbage collection that causes more problems than it's worth at scale, and now being increasingly of the opinion that we made a mistake in doing inheratance in the first place, I think it's time to give the Rustaceans a chance.

OK it's just a little cmdline tool, but it's a nice little thing to get the Rust basics down. 
