[Home]({{ githubPagesUrl }})
[CLI Docs]({{ githubPagesUrl }}cli)
[LIB Docs]({{ githubPagesUrl }}lib)
{{ if: nav.current.prev }}[{{ lua: navlib.get_node(nav, nav.current.prev).title }}]({{ githubPagesUrl }}{{ githubPagesDocsPath }}{{ lua: navlib.get_node(nav, nav.current.prev).path }}){{ fi }}
{{ if: nav.current.next }}[{{ lua: navlib.get_node(nav, nav.current.next).title }}]({{ githubPagesUrl }}{{ githubPagesDocsPath }}{{ lua: navlib.get_node(nav, nav.current.next).path }}){{ fi }}