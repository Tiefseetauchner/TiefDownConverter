[Home]({{ githubPagesUrl }})
[CLI Docs]({{ githubPagesUrl }}cli)
[LIB Docs]({{ githubPagesUrl }}lib)
{{ if: nav.current.prev and meta.githubPagesDocsPath ~= '/' }}[{{ lua: navlib.get_node(nav, nav.current.prev).title }}]({{ lua: navlib.get_node(nav, nav.current.prev).path }}){{ fi }}
{{ if: nav.current.next and meta.githubPagesDocsPath ~= '/' }}[{{ lua: navlib.get_node(nav, nav.current.next).title }}]({{ lua: navlib.get_node(nav, nav.current.next).path }}){{ fi }}