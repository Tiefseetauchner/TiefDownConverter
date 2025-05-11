function Header(elem)
    local h_count = string.rep("h", elem.level)
    return pandoc.RawBlock("latex", "\\" .. h_count .. "{" .. pandoc.utils.stringify(elem.content) .. "}")
  end
  