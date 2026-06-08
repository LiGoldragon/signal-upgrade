# signal-upgrade

`signal-upgrade` is the ordinary Signal contract for the `upgrade`
triad.

It merges the former catalogue-inspection surface from
`signal-sema-upgrade` and the adjacent-version handover surface from
`signal-version-handover`. Runtime migration logic remains in
`upgrade`; this crate only owns the schema-derived wire records and
frame shape.
