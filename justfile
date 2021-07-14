tclean:
    cargo insta test --delete-unreferenced-snapshots

review:
    cargo insta review