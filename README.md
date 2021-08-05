# Flatter

A command line utility for rendering SVGs to PNGs for Flutter applications.

Flutter does not natively support SVGs, and 3rd-party packages to provide SVG support have historically had issues with certain types of SVGs.

Often, however, SVGs are available ahead of time (e.g. provided by a designer) and can be "flattened" into PNGs, which Flutter natively supports. Flatter aims to provide a fast, accurate, and repeatable renders of static SVGs, optimized for this use case.
