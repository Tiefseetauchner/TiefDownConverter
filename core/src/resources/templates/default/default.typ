#import "./meta.typ": title,subtitle,authors
#set page(paper:"a4")

#align(center + top)[
  #text(size: 27.5pt)[#title]
  #linebreak()
  #v(5pt)
  #text(size: 15pt)[#subtitle]
]

#align(center + bottom)[#text(size: 13pt)[#authors]]

#pagebreak()

#include "./output.typ"