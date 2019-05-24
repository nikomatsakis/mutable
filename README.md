# mutable

This repository contains a thought experiment around how to make
working with shared, mutable data more ergonomic. My goal is to enable
a "Java-like" model -- basically one where you cannot get a reference
to a field, but instead you only have the option to read the field
value or set it. Moreover, fields may be *either* of primitive type or
of some kind of garbage collected value -- in our case, we use `Rc` or
`Arc` as a "stand-in" for garbage collected values.


