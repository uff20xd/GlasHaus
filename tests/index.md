@tags library;
@alias main;

# What is GlasHaus?
GlasHaus is an extension of Markdown with easy integration into any text editor and native compilation to html.

# How does it work?
GlasHaus is a server that runs in the Background like an LSP.
Only that its protocol ist far more rudimentary and has extra functions like tagging.

# HTML Integration?
Yes, you can create jumpable links like this: @[name|section]
During compilation they will be transformed into links.
You can also embed pictures: @pic(xdim, ydim) name;
