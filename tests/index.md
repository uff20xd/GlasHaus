@tags library;
@alias main;

# What is GlasHaus?
GlasHaus is an extension of Markdown with easy integration into any text editor and native compilation to html.

# How does it work?
GlasHaus is a server that runs in the Background like an LSP.
Only that its protocol ist far more rudimentary and has extra functions like tagging.

# HTML Integration?
Yes, you can create jumpable links like this: 
> @[name|section]
During compilation they will be transformed into links.


You can also embed pictures:
> @pic(xdim, ydim) name;

# Why not use the Integrated Markdown features for this?
Things like file linking and picture linking kind of sucks in regular markdown and I
want GlasHaus to be more akin to Obsidian with its spidernet-like flat structure, which
works well with tagging. You still can have namespaces, but its an explicit thing and not the 
standard.
