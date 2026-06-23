[
  (closure)
  (map)
  (list)
  (argument_list)
  (parameter_list)
  (for_parameters)
] @indent

((for_loop
  body: (_) @_body) @indent
  (#not-has-type? @_body closure))
; TODO: while, try

(closure "}" @outdent)
(argument_list ")" @outdent)
(for_parameters ")" @outdent)
(list "]" @outdent)
(map "]" @outdent)
