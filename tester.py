from python.fburpp_python import FileType, fburpp


job = fburpp(FileType.CSV).read(
    path="/home/kyle/projects/fburpp/rust/example.csv",
    col_names=["foo", "bar", "baz"],
    col_types=[str, int, str],
).select(
    columns=["foo", "baz"]
).write(
    path="/home/kyle/projects/fburpp/rust/example.out.csv"
)

job.run()
